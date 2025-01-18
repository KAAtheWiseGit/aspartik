#![allow(dead_code)]

use vulkano::{
	buffer::{Buffer, BufferCreateInfo, BufferUsage},
	command_buffer::{
		allocator::StandardCommandBufferAllocator,
		allocator::StandardCommandBufferAllocatorCreateInfo,
		AutoCommandBufferBuilder, CommandBufferUsage,
	},
	descriptor_set::{
		allocator::StandardDescriptorSetAllocator,
		PersistentDescriptorSet, WriteDescriptorSet,
	},
	device::{
		Device, DeviceCreateInfo, Features, Queue, QueueCreateInfo,
		QueueFlags,
	},
	instance::{Instance, InstanceCreateInfo},
	memory::allocator::{
		AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator,
	},
	pipeline::{
		compute::ComputePipelineCreateInfo,
		layout::PipelineDescriptorSetLayoutCreateInfo, ComputePipeline,
		Pipeline, PipelineBindPoint, PipelineLayout,
		PipelineShaderStageCreateInfo,
	},
	sync::{self, GpuFuture},
	VulkanLibrary,
};

use std::sync::Arc;

use super::{Likelihood, Row};
use base::substitution::Substitution;

pub struct GpuLikelihood<const N: usize> {
	// TODO: bench and see if allocators and such should be preserved here
	// buffers:
	// - one array for final likelihoods
	// - number of nodes
	device: Arc<Device>,
	queue: Arc<Queue>,
	updated_nodes: Vec<usize>,

	num_nodes: usize,
	num_sites: usize,
}

impl<const N: usize> Likelihood for GpuLikelihood<N> {
	type Row = Row<N>;
	type Substitution = Substitution<N>;

	fn propose(
		&mut self,
		nodes: &[usize],
		substitutions: &[Self::Substitution],
		children: &[usize],
	) {
		// load the buffers:
		// - nodes (uint (convert to u32))
		// - substitutions (dmat4x4)
		// - children (uint (convert to u32))
		// - length of all of the above
		//
		// update the values with width of 32.  (TODO: what to do with
		// out of bounds?)
	}

	fn likelihood(&self) -> f64 {
		// load the Likelihood buffer and ln and sum it
		todo!()
	}

	fn accept(&mut self) {
		// clear `updated_nodes`
		todo!()
	}

	fn reject(&mut self) {
		// load `updated_nodes` into a buffer and switch all of the
		// pointers in the device probabilities buffer
		todo!()
	}
}

impl<const N: usize> GpuLikelihood<N> {
	pub fn new(mut sites: Vec<Vec<Row<N>>>) -> Self {
		let num_sites = sites.len();
		let num_nodes = sites[0].len();

		// TODO: ShchurVec-like double length structure (+ pointers).
		// The update status is not needed, as it resides in
		// `updated_nodes`
		let num_internas = num_nodes - 1;
		let mut probabilities = vec![];
		for column in &mut sites {
			probabilities.append(column);
			probabilities.append(&mut vec![
				Row::<N>::default();
				num_internas
			]);
		}

		let library = VulkanLibrary::new().unwrap();

		let instance =
			Instance::new(library, InstanceCreateInfo::default())
				.unwrap();

		let physical_device = instance
			.enumerate_physical_devices()
			.unwrap()
			.next()
			.unwrap();

		let queue_family_index = physical_device
			.queue_family_properties()
			.iter()
			.enumerate()
			.position(|(_, queue_family_properties)| {
				queue_family_properties
					.queue_flags
					.contains(QueueFlags::GRAPHICS)
			})
			.unwrap() as u32;

		let (device, mut queues) = Device::new(
			physical_device,
			DeviceCreateInfo {
				// here we pass the desired queue family to use by index
				queue_create_infos: vec![QueueCreateInfo {
					queue_family_index,
					..Default::default()
				}],
				enabled_features: Features {
					shader_float64: true,
					..Default::default()
				},
				..Default::default()
			},
		)
		.unwrap();

		let queue = queues.next().unwrap();

		let memory_allocator = Arc::new(
			StandardMemoryAllocator::new_default(device.clone()),
		);

		let data_buffer = Buffer::from_iter(
			memory_allocator.clone(),
			BufferCreateInfo {
				usage: BufferUsage::STORAGE_BUFFER,
				..Default::default()
			},
			AllocationCreateInfo {
				memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
					| MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
				..Default::default()
			},
			probabilities.clone(),
		).unwrap();

		mod cs {
			vulkano_shaders::shader! {
				ty: "compute",
				src: "
					#version 460

					layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

					layout(set = 0, binding = 0) buffer Data {
						dvec4 probabilities[];
					} buf;

					void main() {
						uint idx = gl_GlobalInvocationID.x;
						buf.probabilities[idx] *= 2.0;
					}
				"
			}
		}

		let shader = cs::load(device.clone()).unwrap();

		let cs = shader.entry_point("main").unwrap();
		let stage = PipelineShaderStageCreateInfo::new(cs);
		let layout = PipelineLayout::new(
			device.clone(),
			PipelineDescriptorSetLayoutCreateInfo::from_stages([
				&stage,
			])
			.into_pipeline_layout_create_info(device.clone())
			.unwrap(),
		)
		.unwrap();

		let compute_pipeline = ComputePipeline::new(
			device.clone(),
			None,
			ComputePipelineCreateInfo::stage_layout(stage, layout),
		)
		.unwrap();

		let descriptor_set_allocator =
			StandardDescriptorSetAllocator::new(
				device.clone(),
				Default::default(),
			);

		let pipeline_layout = compute_pipeline.layout();
		let descriptor_set_layouts = pipeline_layout.set_layouts();
		let descriptor_set_layout_index = 0;
		let descriptor_set_layout = descriptor_set_layouts
			.get(descriptor_set_layout_index)
			.unwrap();

		let descriptor_set = PersistentDescriptorSet::new(
			&descriptor_set_allocator,
			descriptor_set_layout.clone(),
			[WriteDescriptorSet::buffer(0, data_buffer.clone())], // 0 is the binding
			[],
		)
		.unwrap();

		let command_buffer_allocator = StandardCommandBufferAllocator::new(device.clone(), StandardCommandBufferAllocatorCreateInfo::default());

		let mut command_buffer_builder =
			AutoCommandBufferBuilder::primary(
				&command_buffer_allocator,
				queue.queue_family_index(),
				CommandBufferUsage::OneTimeSubmit,
			)
			.unwrap();

		let num_groups = (probabilities.len() + 63) / 64;
		let work_group_counts = [num_groups as u32, 1, 1];

		command_buffer_builder
			.bind_pipeline_compute(compute_pipeline.clone())
			.unwrap()
			.bind_descriptor_sets(
				PipelineBindPoint::Compute,
				compute_pipeline.layout().clone(),
				descriptor_set_layout_index as u32,
				descriptor_set,
			)
			.unwrap()
			.dispatch(work_group_counts)
			.unwrap();

		let command_buffer = command_buffer_builder.build().unwrap();

		let future = sync::now(device.clone())
			.then_execute(queue.clone(), command_buffer)
			.unwrap()
			.then_signal_fence_and_flush()
			.unwrap();

		future.wait(None).unwrap();

		let content = data_buffer.read().unwrap();
		for (modified, old) in content.iter().zip(probabilities) {
			assert_eq!(*modified, old * 2.0);
		}

		GpuLikelihood {
			device,
			queue,
			updated_nodes: Vec::new(),
			num_nodes,
			num_sites,
		}
	}
}

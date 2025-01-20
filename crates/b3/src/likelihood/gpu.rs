#![allow(unused)]

use vulkano::{
	buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
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
use linalg::Vector;

pub struct GpuLikelihood<const N: usize> {
	// TODO: bench and see if allocators and such should be preserved here
	// buffers:
	// - one array for final likelihoods
	// - number of nodes
	device: Arc<Device>,
	queue: Arc<Queue>,

	probabilities: Subbuffer<[Vector<f64, N>]>,
	masks: Subbuffer<[u32]>,

	/// Unlike in the CPU likelihood, this field is essential.  It tracks
	/// which nodes were updated in the on-GPU buffer.  As such, it acts as
	/// the `edited` field in `ShchurVec`.
	updated_nodes: Vec<usize>,

	num_leaves: usize,
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

		mod cs {
			vulkano_shaders::shader! {
				ty: "compute",
				path: "src/likelihood/propose.glsl",
			}
		}

		let shader = cs::load(self.device.clone()).unwrap();

		let cs = shader.entry_point("main").unwrap();
		let stage = PipelineShaderStageCreateInfo::new(cs);
		let layout = PipelineLayout::new(
			self.device.clone(),
			PipelineDescriptorSetLayoutCreateInfo::from_stages([
				&stage,
			])
			.into_pipeline_layout_create_info(self.device.clone())
			.unwrap(),
		)
		.unwrap();

		let compute_pipeline = ComputePipeline::new(
			self.device.clone(),
			None,
			ComputePipelineCreateInfo::stage_layout(stage, layout),
		)
		.unwrap();

		let memory_allocator =
			Arc::new(StandardMemoryAllocator::new_default(
				self.device.clone(),
			));

		let descriptor_set_allocator =
			StandardDescriptorSetAllocator::new(
				self.device.clone(),
				Default::default(),
			);

		let num_rows_buffer = Buffer::from_data(
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
			(self.num_leaves * 2 - 1) as u32,
		).unwrap();

		let pipeline_layout = compute_pipeline.layout();
		let descriptor_set_layouts = pipeline_layout.set_layouts();

		let descriptor_set_layout_0 =
			descriptor_set_layouts.get(0).unwrap();
		let descriptor_set_0 = PersistentDescriptorSet::new(
			&descriptor_set_allocator,
			descriptor_set_layout_0.clone(),
			[
				WriteDescriptorSet::buffer(0, num_rows_buffer),
				WriteDescriptorSet::buffer(
					1,
					self.probabilities.clone(),
				),
				WriteDescriptorSet::buffer(
					2,
					self.masks.clone(),
				),
			],
			[],
		)
		.unwrap();

		let nodes_buffer = Buffer::from_iter(
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
			nodes.to_vec().clone(),
		).unwrap();
		let substitutions_buffer = Buffer::from_iter(
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
			substitutions.to_vec().clone(),
		).unwrap();
		let children_buffer = Buffer::from_iter(
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
			children.to_vec().clone(),
		).unwrap();

		let descriptor_set_layout_1 =
			descriptor_set_layouts.get(1).unwrap();
		let descriptor_set_1 = PersistentDescriptorSet::new(
			&descriptor_set_allocator,
			descriptor_set_layout_1.clone(),
			[
				WriteDescriptorSet::buffer(0, nodes_buffer),
				WriteDescriptorSet::buffer(
					1,
					substitutions_buffer,
				),
				WriteDescriptorSet::buffer(2, children_buffer),
			],
			[],
		)
		.unwrap();

		let cmd_buf_allocator = StandardCommandBufferAllocator::new(
			self.device.clone(),
			StandardCommandBufferAllocatorCreateInfo::default(),
		);

		let mut command_buffer_builder =
			AutoCommandBufferBuilder::primary(
				&cmd_buf_allocator,
				self.queue.queue_family_index(),
				CommandBufferUsage::OneTimeSubmit,
			)
			.unwrap();

		let num_groups = (self.num_sites + 63) / 64;
		let work_group_counts = [num_groups as u32, 1, 1];

		command_buffer_builder
			.bind_pipeline_compute(compute_pipeline.clone())
			.unwrap()
			.bind_descriptor_sets(
				PipelineBindPoint::Compute,
				compute_pipeline.layout().clone(),
				0u32,
				descriptor_set_0,
			)
			.unwrap()
			.bind_descriptor_sets(
				PipelineBindPoint::Compute,
				compute_pipeline.layout().clone(),
				1u32,
				descriptor_set_1,
			)
			.unwrap()
			.dispatch(work_group_counts)
			.unwrap();

		let command_buffer = command_buffer_builder.build().unwrap();

		let future = sync::now(self.device.clone())
			.then_execute(self.queue.clone(), command_buffer)
			.unwrap()
			.then_signal_fence_and_flush()
			.unwrap();

		future.wait(None).unwrap();

		let content = self.probabilities.clone();
		let content = content.read().unwrap();
		for el in content.iter() {
			println!("{:.02}", el);
		}
	}

	fn likelihood(&self) -> f64 {
		// load the Likelihood buffer and ln and sum it
		todo!("likelihood")
	}

	fn accept(&mut self) {
		// `propose` changes the state to how it should be after the
		// update, so this is all what's needed to accept.
		self.updated_nodes.clear();
	}

	fn reject(&mut self) {
		// load `updated_nodes` into a buffer and switch all of the
		// pointers in the device probabilities buffer
		todo!("reject")
	}
}

impl<const N: usize> GpuLikelihood<N> {
	pub fn new(mut sites: Vec<Vec<Row<N>>>) -> Self {
		let num_sites = sites.len();
		let num_leaves = sites[0].len();

		let num_internals = num_leaves - 1;
		// A ShchurVec-like structure
		let mut probabilities = vec![];
		// The mask for probabilities.  32-bit integer in the smallest
		// int type on the GPU.
		let mut masks: Vec<u32> = vec![];
		for column in sites {
			for row in column {
				masks.push(0);
				probabilities.push(row);
				probabilities.push(Row::default());
			}
			for _ in 0..num_internals {
				masks.push(0);
				probabilities.push(Row::default());
				probabilities.push(Row::default());
			}
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

		let probabilities_buffer = Buffer::from_iter(
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

		let masks_buffer = Buffer::from_iter(
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
			masks.clone(),
		).unwrap();

		GpuLikelihood {
			device,
			queue,

			probabilities: probabilities_buffer,
			masks: masks_buffer,

			updated_nodes: Vec::new(),

			num_leaves,
			num_sites,
		}
	}
}

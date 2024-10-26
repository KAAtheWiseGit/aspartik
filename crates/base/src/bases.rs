pub(crate) trait Base: Copy {
	fn as_u8(&self) -> u8;
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DnaNucleoBase {
	Guanine = 0b0001,
	Adenine = 0b0010,
	Thymine = 0b0100,
	Cytosine = 0b1000,

	Purine = 0b0011,
	Pyrimidine = 0b1100,
	Amino = 0b1010,
	Keto = 0b0101,
	Strong = 0b1001,
	Weak = 0b0110,
	NotGuanine = 0b1110,
	NotAdenine = 0b1101,
	NotThymine = 0b1011,
	NotCytosine = 0b0111,

	Any = 0b1111,

	Gap = 0b1_0000,
}

impl Base for DnaNucleoBase {
	fn as_u8(&self) -> u8 {
		*self as u8
	}
}

impl DnaNucleoBase {
	pub fn complement(&self) -> Self {
		match self {
			Self::Guanine => Self::Cytosine,
			Self::Adenine => Self::Thymine,
			Self::Thymine => Self::Adenine,
			Self::Cytosine => Self::Guanine,

			Self::Purine => Self::Pyrimidine,
			Self::Pyrimidine => Self::Purine,
			Self::Amino => Self::Keto,
			Self::Keto => Self::Amino,
			Self::Strong => Self::Weak,
			Self::Weak => Self::Strong,

			Self::NotGuanine => Self::NotCytosine,
			Self::NotAdenine => Self::NotThymine,
			Self::NotThymine => Self::NotAdenine,
			Self::NotCytosine => Self::NotGuanine,

			Self::Any => Self::Any,
			Self::Gap => Self::Gap,
		}
	}

	pub fn includes(&self, other: &Self) -> bool {
		(self.as_u8() & other.as_u8()) == other.as_u8()
	}
}

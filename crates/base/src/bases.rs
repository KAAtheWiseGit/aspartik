use crate::Error;

pub(crate) trait Base: Into<u8> + Copy {
	fn as_u8(&self) -> u8 {
		(*self).into()
	}
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

impl From<DnaNucleoBase> for u8 {
	fn from(value: DnaNucleoBase) -> Self {
		value as u8
	}
}

impl Base for DnaNucleoBase {}

impl TryFrom<char> for DnaNucleoBase {
	type Error = Error;

	// https://genome.ucsc.edu/goldenPath/help/iupac.html
	fn try_from(value: char) -> Result<Self, Self::Error> {
		Ok(match value {
			'G' => Self::Guanine,
			'A' => Self::Adenine,
			'T' => Self::Thymine,
			'C' => Self::Cytosine,

			'R' => Self::Purine,
			'Y' => Self::Pyrimidine,
			'M' => Self::Amino,
			'K' => Self::Keto,
			'S' => Self::Strong,
			'W' => Self::Weak,
			'H' => Self::NotGuanine,
			'B' => Self::NotAdenine,
			'V' => Self::NotThymine,
			'D' => Self::NotCytosine,

			'N' => Self::Any,
			'-' => Self::Gap,

			_ => Err(Error::InvalidNucleoBaseChar(value))?,
		})
	}
}

impl From<DnaNucleoBase> for char {
	fn from(value: DnaNucleoBase) -> char {
		match value {
			DnaNucleoBase::Guanine => 'G',
			DnaNucleoBase::Adenine => 'A',
			DnaNucleoBase::Thymine => 'T',
			DnaNucleoBase::Cytosine => 'C',

			DnaNucleoBase::Purine => 'R',
			DnaNucleoBase::Pyrimidine => 'Y',
			DnaNucleoBase::Amino => 'M',
			DnaNucleoBase::Keto => 'K',
			DnaNucleoBase::Strong => 'S',
			DnaNucleoBase::Weak => 'W',
			DnaNucleoBase::NotGuanine => 'H',
			DnaNucleoBase::NotAdenine => 'B',
			DnaNucleoBase::NotThymine => 'V',
			DnaNucleoBase::NotCytosine => 'D',

			DnaNucleoBase::Any => 'N',
			DnaNucleoBase::Gap => '-',
		}
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

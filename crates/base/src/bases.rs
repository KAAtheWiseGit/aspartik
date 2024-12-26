use anyhow::Error;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum NucleoBaseError {
	#[error("'{0}' not a valid IUPAC nucleobase character")]
	InvalidChar(char),
	#[error("{0:X} is not a valid nucleo base binary code")]
	InvalidByte(u8),
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DnaNucleoBase {
	Adenine = 0b0001,
	Cytosine = 0b0010,
	Guanine = 0b0100,
	Thymine = 0b1000,

	Weak = 0b1001,
	Strong = 0b0110,
	Amino = 0b1100,
	Ketone = 0b0011,
	Purine = 0b0101,
	Pyrimidine = 0b1010,

	NotAdenine = 0b1110,
	NotCytosine = 0b1101,
	NotGuanine = 0b1011,
	NotThymine = 0b0111,

	Any = 0b1111,

	Gap = 0b1_0000,
}

impl From<DnaNucleoBase> for u8 {
	fn from(value: DnaNucleoBase) -> Self {
		value as u8
	}
}

impl TryFrom<u8> for DnaNucleoBase {
	type Error = Error;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		Ok(match value {
			0b0001 => Self::Adenine,
			0b0010 => Self::Cytosine,
			0b0100 => Self::Guanine,
			0b1000 => Self::Thymine,

			0b1001 => Self::Weak,
			0b0110 => Self::Strong,
			0b1100 => Self::Amino,
			0b0011 => Self::Ketone,
			0b0101 => Self::Purine,
			0b1010 => Self::Pyrimidine,

			0b1110 => Self::NotAdenine,
			0b1101 => Self::NotCytosine,
			0b1011 => Self::NotGuanine,
			0b0111 => Self::NotThymine,

			0b1111 => Self::Any,

			0b1_0000 => Self::Gap,

			_ => Err(NucleoBaseError::InvalidByte(value))?,
		})
	}
}

impl TryFrom<char> for DnaNucleoBase {
	type Error = Error;

	// https://genome.ucsc.edu/goldenPath/help/iupac.html
	fn try_from(value: char) -> Result<Self, Self::Error> {
		Ok(match value {
			'A' => Self::Adenine,
			'C' => Self::Cytosine,
			'G' => Self::Guanine,
			'T' => Self::Thymine,

			'W' => Self::Weak,
			'S' => Self::Strong,
			'M' => Self::Amino,
			'K' => Self::Ketone,
			'R' => Self::Purine,
			'Y' => Self::Pyrimidine,

			'B' => Self::NotAdenine,
			'D' => Self::NotCytosine,
			'H' => Self::NotGuanine,
			'V' => Self::NotThymine,

			'N' => Self::Any,
			'-' => Self::Gap,

			_ => Err(NucleoBaseError::InvalidChar(value))?,
		})
	}
}

impl From<DnaNucleoBase> for char {
	fn from(value: DnaNucleoBase) -> char {
		match value {
			DnaNucleoBase::Adenine => 'A',
			DnaNucleoBase::Cytosine => 'C',
			DnaNucleoBase::Guanine => 'G',
			DnaNucleoBase::Thymine => 'T',

			DnaNucleoBase::Weak => 'W',
			DnaNucleoBase::Strong => 'S',
			DnaNucleoBase::Amino => 'M',
			DnaNucleoBase::Ketone => 'K',
			DnaNucleoBase::Purine => 'R',
			DnaNucleoBase::Pyrimidine => 'Y',

			DnaNucleoBase::NotAdenine => 'B',
			DnaNucleoBase::NotCytosine => 'D',
			DnaNucleoBase::NotGuanine => 'H',
			DnaNucleoBase::NotThymine => 'V',

			DnaNucleoBase::Any => 'N',
			DnaNucleoBase::Gap => '-',
		}
	}
}

impl DnaNucleoBase {
	fn as_u8(&self) -> u8 {
		*self as u8
	}

	pub fn complement(&self) -> Self {
		match self {
			Self::Adenine => Self::Thymine,
			Self::Cytosine => Self::Guanine,
			Self::Guanine => Self::Cytosine,
			Self::Thymine => Self::Adenine,

			Self::Weak => Self::Strong,
			Self::Strong => Self::Weak,
			Self::Amino => Self::Ketone,
			Self::Ketone => Self::Amino,
			Self::Purine => Self::Pyrimidine,
			Self::Pyrimidine => Self::Purine,

			Self::NotAdenine => Self::NotThymine,
			Self::NotCytosine => Self::NotGuanine,
			Self::NotGuanine => Self::NotCytosine,
			Self::NotThymine => Self::NotAdenine,

			Self::Any => Self::Any,
			Self::Gap => Self::Gap,
		}
	}

	pub fn includes(&self, other: &Self) -> bool {
		(self.as_u8() & other.as_u8()) == other.as_u8()
	}
}

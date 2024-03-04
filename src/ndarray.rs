use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]

pub enum SingleValue {
	Integer(i32),
	Real(f32),
	Char(char),
}

#[derive(Clone)]
pub enum NDArray {
	SingleValue(SingleValue),
	NDArray {
		shape: Vec<usize>,
		inner: Vec<NDArray>,
	},
}

// you can compare fully single value between them if theyre the same type
// for ndarrays, you can just say equal or not equal
impl PartialEq for SingleValue {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Integer(x), Self::Integer(y)) => x == y,
			(Self::Real(x), Self::Real(y)) => x == y,
			(Self::Char(x), Self::Char(y)) => x == y,
			_ => false,
		}
	}
}

impl PartialOrd for SingleValue {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match (self, other) {
			(Self::Integer(x), Self::Integer(y)) => x.partial_cmp(y),
			(Self::Real(x), Self::Real(y)) => x.partial_cmp(y),
			(Self::Char(x), Self::Char(y)) => x.partial_cmp(y),
			_ => None,
		}
	}
}

impl Debug for NDArray {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::SingleValue(x) => write!(f, "{:?}", x),
			Self::NDArray { shape, inner } => {
				write!(f, "[")?;
				for i in 0..shape[0] {
					if i != 0 {
						write!(f, ", ")?;
					}
					write!(f, "{:?}", inner[i])?;
				}
				write!(f, "]\n")
			}
		}
	}
}

// normal format printing

impl Display for SingleValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Integer(x) => write!(f, "{}", x),
			Self::Real(x) => write!(f, "{}", x),
			Self::Char(x) => write!(f, "{}", x),
		}
	}
}

impl Display for NDArray {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::SingleValue(x) => write!(f, "{}", x),
			Self::NDArray { shape, inner } => {
				write!(f, "[")?;
				for i in 0..shape[0] {
					if i != 0 {
						write!(f, ", ")?;
					}
					write!(f, "{}", inner[i])?;
				}
				write!(f, "]\n")
			}
		}
	}
}

impl NDArray {
	pub fn from_1d_int(inner: Vec<i32>) -> Self {
		Self::NDArray {
			shape: vec![inner.len()],
			inner: inner
				.into_iter()
				.map(|x| Self::SingleValue(SingleValue::Integer(x)))
				.collect(),
		}
	}

	pub fn from_1d_real(inner: Vec<f32>) -> Self {
		Self::NDArray {
			shape: vec![inner.len()],
			inner: inner
				.into_iter()
				.map(|x| Self::SingleValue(SingleValue::Real(x)))
				.collect(),
		}
	}

	pub fn from_1d_char(inner: Vec<char>) -> Self {
		Self::NDArray {
			shape: vec![inner.len()],
			inner: inner
				.into_iter()
				.map(|x| Self::SingleValue(SingleValue::Char(x)))
				.collect(),
		}
	}

	pub fn get(&self, indices: &[usize]) -> Self {
		match self {
			Self::NDArray { shape, inner } => {
				if indices.len() == 0 {
					panic!("Cannot index into array with no indices");
				}
				if indices.len() == 1 {
					inner[indices[0]].clone()
				} else {
					inner[indices[0]].get(&indices[1..].to_vec())
				}
			}
			_ => panic!("Cannot index into scalar"),
		}
	}

	pub fn set(&mut self, indices: &[usize], value: Self) {
		//println!("indices : {:?}", indices);
		//println!("self : {:?}", self);
		match self {
			Self::NDArray { shape, inner } => {
				if indices.len() == 0 {
					panic!("Cannot index into array with no indices");
				}
				if indices.len() == 1 {
					inner[indices[0]] = value;
				} else {
					inner[indices[0]].set(&indices[1..].to_vec(), value);
				}
			}
			_ => panic!("Cannot index into scalar"),
		}
	}

	pub fn zeros(dims: &[usize]) -> Self {
		if dims.len() == 0 {
			return Self::SingleValue(SingleValue::Integer(0));
		}
		if dims.len() == 1 {
			Self::NDArray {
				shape: dims.to_vec(),
				inner: vec![Self::SingleValue(SingleValue::Integer(0)); dims[0]],
			}
		} else {
			Self::NDArray {
				shape: dims.to_vec(),
				inner: vec![Self::zeros(&dims[1..].to_vec()); dims[0]],
			}
		}
	}

	pub fn get_single_value(&self) -> SingleValue {
		match self {
			Self::SingleValue(x) => x.clone(),
			_ => {
				dbg!(self);
				panic!("Not a scalar");
			}
		}
	}

	pub fn shape(&self) -> Vec<usize> {
		match self {
			Self::SingleValue(_) => vec![],
			Self::NDArray { shape, inner } => shape.clone(),
		}
	}

	pub fn addition(a: SingleValue, b: SingleValue) -> NDArray {
		NDArray::SingleValue(match (a, b) {
			(SingleValue::Integer(x), SingleValue::Integer(y)) => SingleValue::Integer(x + y),
			(SingleValue::Real(x), SingleValue::Real(y)) => SingleValue::Real(x + y),
			(SingleValue::Char(x), SingleValue::Char(y)) => panic!("Cannot add two chars"),
			_ => panic!("Cannot add different types"),
		})
	}

	pub fn substraction(a: SingleValue, b: SingleValue) -> NDArray {
		NDArray::SingleValue(match (a, b) {
			(SingleValue::Integer(x), SingleValue::Integer(y)) => SingleValue::Integer(x - y),
			(SingleValue::Real(x), SingleValue::Real(y)) => SingleValue::Real(x - y),
			(SingleValue::Char(x), SingleValue::Char(y)) => panic!("Cannot substract two chars"),
			_ => panic!("Cannot substract different types"),
		})
	}

	pub fn division(a: SingleValue, b: SingleValue) -> NDArray {
		NDArray::SingleValue(match (a, b) {
			(SingleValue::Integer(x), SingleValue::Integer(y)) => SingleValue::Integer(x / y),
			(SingleValue::Real(x), SingleValue::Real(y)) => SingleValue::Real(x / y),
			(SingleValue::Char(x), SingleValue::Char(y)) => panic!("Cannot divide two chars"),
			_ => panic!("Cannot divide different types"),
		})
	}

	pub fn get_integer(&self) -> i32 {
		match self {
			Self::SingleValue(SingleValue::Integer(x)) => *x,
			_ => panic!("Not an integer"),
		}
	}

	pub fn get_char(&self) -> char {
		match self {
			Self::SingleValue(SingleValue::Char(x)) => *x,
			_ => panic!("Not a char"),
		}
	}

	pub fn get_real(&self) -> f32 {
		match self {
			Self::SingleValue(SingleValue::Real(x)) => *x,
			_ => panic!("Not a real"),
		}
	}

	pub fn from_vec(inner: Vec<NDArray>) -> Self {
		Self::NDArray {
			shape: vec![inner.len()],
			inner,
		}
	}
}

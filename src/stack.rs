use core::fmt::{Debug, Formatter};
use std::ops::Deref;

use crate::ndarray::NDArray;

pub struct Stack {
	pub inner: Vec<NDArray>,
}

impl Debug for Stack {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "--- Bottom ---")?;
		for item in self.inner.iter() {
			write!(f, "\n{:?}", item)?;
		}
		write!(f, "\n--- Top ---")?;
		Ok(())
	}
}

impl Stack {
	pub fn new() -> Self {
		Self { inner: Vec::new() }
	}

	pub fn push(&mut self, value: NDArray) {
		self.inner.push(value);
	}

	pub fn pop(&mut self) -> Option<NDArray> {
		self.inner.pop()
	}

	pub fn peek(&self) -> Option<&NDArray> {
		self.inner.last()
	}

	pub fn from_vec(inner: Vec<NDArray>) -> Self {
		Self { inner }
	}
}

impl Deref for Stack {
	type Target = Vec<NDArray>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

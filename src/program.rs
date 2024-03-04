use crate::ndarray::{NDArray, SingleValue};
use crate::stack::Stack;
use std::mem;
use std::ops::Deref;

use crate::token::*;

#[derive(Debug)]
pub struct Program {
	pub stack: Stack,
	pub current_instruction: usize,
	pub indices_current: Vec<usize>,
}

impl Program {
	pub fn new(source: &str) -> Self {
		let mut stack = Stack::new();
		let mut code = String::from(source);
		Self::preprocess(&mut code);
		let code = code.chars().collect::<Vec<char>>();
		let indices_current = vec![0];
		stack.push(NDArray::from_1d_char(code));
		Self {
			stack,
			current_instruction: 0,
			indices_current,
		}
	}

	//same stack, different code, same current instruction
	pub fn subprogram(code: &str, parent_program: &Self) -> Self {
		let mut code = String::from(code);
		Self::preprocess(&mut code);
		let code = code.chars().collect::<Vec<char>>();
		let mut new_stack = Stack::new();
		new_stack.push(NDArray::from_1d_char(code));

		// clone all the other things from the stack except the code
		for i in 1..parent_program.stack.len() {
			new_stack.push(parent_program.stack[i].clone());
		}
		let indices_current = parent_program.indices_current.clone();
		Self {
			stack: new_stack,
			current_instruction: 0,
			indices_current,
		}
	}

	pub fn push(&mut self, value: NDArray) {
		self.stack.push(value);
		self.indices_current[0] += 1;
	}

	pub fn pop(&mut self) -> Option<NDArray> {
		self.indices_current[0] -= 1;
		self.stack.pop()
	}

	pub fn load(path: &str) -> Self {
		let mut code = String::new();
		std::fs::read_to_string(path)
			.unwrap()
			.chars()
			.for_each(|c| {
				code.push(c);
			});
		Self::new(&code)
	}

	pub fn preprocess(code: &mut String) {
		const MACROS: [(&str, &str); 1] = [("6", "6")];
		for (original, expanded) in MACROS.iter() {
			*code = code.replace(original, expanded);
		}
	}

	pub fn get_code(&self) -> String {
		let code = self.stack[0].clone();
		let code = match code {
			NDArray::SingleValue(_) => panic!("Cannot get code from scalar"),
			NDArray::NDArray { shape, inner } => inner,
		};
		let mut code_string = String::new();

		for x in code {
			if let NDArray::SingleValue(SingleValue::Char(x)) = x {
				code_string.push(x);
			}
		}

		return code_string;
	}

	pub fn execute(&mut self) {
		let tokens = tokenize(&self.get_code());
		for token in tokens {
			match token {
				Token::Integer(x) => self.push(NDArray::SingleValue(SingleValue::Integer(x))),
				Token::Real(x) => self.push(NDArray::SingleValue(SingleValue::Real(x))),
				Token::Char(x) => self.push(NDArray::SingleValue(SingleValue::Char(x))),
				Token::String(x) => {
					let code = x.chars().collect::<Vec<char>>();
					self.push(NDArray::from_1d_char(code));
				}
				Token::Set(set) => self.push(NDArray::SingleValue(SingleValue::Char(match set {
					Set::Integer => 'I',
					Set::Real => 'R',
					Set::Char => 'C',
				}))),
				Token::Conversion => {
					//take the top of the stack to know what to convert to
					let set = match self.pop().unwrap() {
						NDArray::SingleValue(SingleValue::Char(x)) => match x {
							'I' => Set::Integer,
							'R' => Set::Real,
							'C' => Set::Char,
							_ => panic!("Cannot convert to unknown type"),
						},
						_ => panic!("Cannot convert to unknown type"),
					};
					//take the next element to know what to convert
					let to_convert = self.pop().unwrap();
					// if its a single value
					if let NDArray::SingleValue(x) = to_convert {
						let converted = match x {
							SingleValue::Integer(x) => match set {
								Set::Integer => SingleValue::Integer(x),
								Set::Real => SingleValue::Real(x as f32),
								Set::Char => {
									SingleValue::Char(unsafe { mem::transmute::<i32, char>(x) })
								}
								_ => panic!("Cannot convert to unknown type"),
							},
							SingleValue::Real(x) => match set {
								Set::Integer => SingleValue::Integer(x as i32),
								Set::Real => SingleValue::Real(x),
								Set::Char => panic!("Cannot convert real to char"),
								_ => panic!("Cannot convert to unknown type"),
							},
							SingleValue::Char(x) => match set {
								Set::Integer => {
									SingleValue::Integer(unsafe { mem::transmute::<char, i32>(x) })
								}
								Set::Real => panic!("Cannot convert char to real"),
								Set::Char => SingleValue::Char(x),
								_ => panic!("Cannot convert to unknown type"),
							},
						};
						self.push(NDArray::SingleValue(converted));
					} else {
						// convert the whole array
						let mut array_converted = to_convert.clone();
						let mut indexes = vec![0; array_converted.shape().len()];
						let final_shape = array_converted.shape().clone();
						while (indexes
							.clone()
							.into_iter()
							.map(|x| x + 1)
							.collect::<Vec<_>>() != final_shape)
						{
							let mut to_convert = array_converted.get(&indexes);
							let converted = match to_convert {
								NDArray::SingleValue(x) => match x {
									SingleValue::Integer(x) => match set {
										Set::Integer => SingleValue::Integer(x),
										Set::Real => SingleValue::Real(x as f32),
										Set::Char => SingleValue::Char(unsafe {
											mem::transmute::<i32, char>(x)
										}),
										_ => panic!("Cannot convert to unknown type"),
									},
									SingleValue::Real(x) => match set {
										Set::Integer => SingleValue::Integer(x as i32),
										Set::Real => SingleValue::Real(x),
										Set::Char => panic!("Cannot convert real to char"),
										_ => panic!("Cannot convert to unknown type"),
									},
									SingleValue::Char(x) => match set {
										Set::Integer => SingleValue::Integer(unsafe {
											mem::transmute::<char, i32>(x)
										}),
										Set::Real => panic!("Cannot convert char to real"),
										Set::Char => SingleValue::Char(x),
										_ => panic!("Cannot convert to unknown type"),
									},
								},
								_ => panic!("Cannot convert array to scalar"),
							};
							array_converted.set(&indexes, NDArray::SingleValue(converted));

							let mut index_to_increment = indexes.len() - 1;
							indexes[index_to_increment] += 1;
							while indexes[index_to_increment]
								== array_converted.shape()[index_to_increment]
							{
								indexes[index_to_increment] = 0;
								index_to_increment -= 1;
								indexes[index_to_increment] += 1;
							}
						}
						self.push(array_converted);
					}
				}
				Token::Operator(f) => f(self),
				Token::RealTimeMacro(f) => {
					// create another program made of the same stack
					// but with the code being only the macro
					// then execute that program
					let mut program = Program::new(&f(self));
					program.execute();
					// get the result of the program
					let result = program.stack.pop().unwrap();
					// push the result onto the current program's stack
					self.push(result);
				}
			}
		}
	}

	pub fn get_result(&self) -> NDArray {
		let len = self.stack.len();
		self.stack[len - 1].clone()
	}
}

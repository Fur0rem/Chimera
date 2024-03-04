// Sets are used to define the type of a variable
// which is symbolized by a single character
// N = Integer
// R = Real
// C = Char
// P = Program

use std::mem;

use crate::ndarray::{NDArray, SingleValue};
use crate::program::{self, Program};
use crate::stack::Stack;

#[derive(Debug)]
pub enum Set {
	Integer,
	Real,
	Char,
}

#[derive(Debug)]
pub enum Token {
	Integer(i32),
	Real(f32),
	Char(char),
	String(String),
	Set(Set),
	Conversion,
	// Real time macros expand to other code that'll get executed in another program
	RealTimeMacro(fn(&Program) -> String),
	Operator(fn(&mut Program)),
}

pub fn tokenize(source: &str) -> Vec<Token> {
	let mut tokens = Vec::new();
	let mut chars = source.chars().peekable();
	while let Some(c) = chars.next() {
		match c {
			'v' => {
				//pop
				tokens.push(Token::Operator(|program| {
					program.pop();
				}));
			}
			'/' => {
				//divide
				tokens.push(Token::Operator(|program| {
					let a = program.pop().unwrap();
					let b = program.pop().unwrap();
					let a = a.get_single_value();
					let b = b.get_single_value();
					program.push(NDArray::division(a, b));
				}));
			}
			'§' => {
				tokens.push(Token::Conversion);
			}
			'&' => {
				//get current element
				tokens.push(Token::Operator(|program| {
					//dbg!(&program.indices_current);
					//dbg!(&program.stack);
					//get the current element
					let indices = &program.indices_current;
					let mut current = program.stack[indices[0]].clone();
					for i in indices.iter().skip(1) {
						current = current.get(&[*i]);
					}
					program.push(current);
				}));
			}
			'd' => {
				//debug
				tokens.push(Token::Operator(|program| {
					println!("DEBUG");
					dbg!(&program);
				}));
			}
			'~' => {
				//swap the two top elements
				tokens.push(Token::Operator(|program| {
					let a = program.pop().unwrap();
					let b = program.pop().unwrap();
					program.push(a);
					program.push(b);
				}));
			}
			'=' => {
				//double equal
				let next_char = chars.peek();
				if let Some('=') = next_char {
					chars.next();
					tokens.push(Token::Operator(|program| {
						let a = program.pop().unwrap();
						let b = program.pop().unwrap();
						let a = a.get_single_value();
						let b = b.get_single_value();
						program.push(NDArray::SingleValue(SingleValue::Integer(if a == b {
							1
						} else {
							0
						})));
					}));
				}
				// map operator
				else {
					tokens.push(Token::Operator(|program| {
						let code = program.pop().unwrap();
						let array = program.pop().unwrap();

						//print the code
						let mut program_string = String::new();
						if let NDArray::NDArray { shape, inner } = &code {
							for x in inner {
								match x {
									NDArray::SingleValue(SingleValue::Char(x)) => {
										program_string.push(*x);
									}
									_ => panic!("Expected a program"),
								}
							}
						} else {
							panic!("Expected a program");
						}

						let (array_shape, mut array_inner) =
							if let NDArray::NDArray { shape, inner } = array {
								(shape, inner)
							} else {
								dbg!(array);
								panic!("Expected an array");
							};

						// for each element in the array
						// execute the code

						for i in 0..array_shape[0] {
							program.indices_current.push(i);
							let mut other_program = Program::subprogram(&program_string, program);
							//dbg!(&other_program);
							other_program.execute();
							let result_element = other_program.get_result();
							// set the result in the new array
							array_inner[i] = result_element;
							program.indices_current.pop();
						}

						program.push(NDArray::NDArray {
							shape: array_shape.clone(),
							inner: array_inner,
						});
					}));
				}
			}
			// reshape operator
			'^' => {
				tokens.push(Token::Operator(|program| {
					let nb_dims = program.pop().unwrap().get_integer();
					let mut shape = Vec::new();
					for _ in 0..nb_dims {
						let dim = program.pop().unwrap().get_integer();
						shape.push(dim as usize);
					}
					let mut values = Vec::new();
					for _ in 0..shape.iter().product() {
						values.push(program.pop().unwrap());
					}

					let mut array = NDArray::zeros(&shape);
					let total = shape.iter().product();
					for i in (0..total).rev() {
						let mut indices = vec![0; shape.len()];
						let mut index = i;
						for j in 0..shape.len() {
							let dim = shape[j];
							indices[j] = index % dim;
							index /= dim;
						}
						let indices = indices.iter().rev().map(|x| *x).collect::<Vec<_>>();
						array.set(&indices, values[total - i - 1].clone());
					}
					program.push(array);
				}));
			}
			// fold left operator with window
			'¨' => {
				tokens.push(Token::Operator(|program| {
					let operation = program.pop().unwrap();
					let window_size = program.pop().unwrap().get_integer();
					let identity = program.pop().unwrap();
					let array = program.pop().unwrap();

					let mut array = if let NDArray::NDArray { shape, inner } = array {
						shape;
						inner
					} else {
						panic!("Expected an array");
					};

					let mut result = identity;
					let mut window = Vec::new();
					for i in 0..array.len() {
						window.push(array[i].clone());
						if window.len() == window_size as usize {
							let mut program_string = String::new();
							if let NDArray::NDArray { shape, inner } = &operation {
								for x in inner {
									match x {
										NDArray::SingleValue(SingleValue::Char(x)) => {
											program_string.push(*x);
										}
										_ => panic!("Expected a program"),
									}
								}
							} else {
								panic!("Expected a program");
							}
							let mut other_program = Program::subprogram(&program_string, program);
							other_program.stack.push(result);
							for x in window.iter().rev() {
								other_program.stack.push(x.clone());
							}
							other_program.execute();
							result = other_program.get_result();
							window.remove(0);
						}
					}

					program.push(result);
				}));
			}
			'@' => {
				tokens.push(Token::Operator(|program| {
					//dbg!(&program.stack);
					let indices = program.pop().unwrap();
					if let NDArray::SingleValue(SingleValue::Integer(x)) = indices {
						let value = program.stack.get(x as usize).unwrap().clone();
						program.push(value);
						return;
					}
					//get the current element
					let mut current =
						program.stack[indices.get(&[0]).get_integer() as usize].clone();
					//dbg!(&current);
					for i in 1..indices.shape()[0] {
						current = current.get(&[indices.get(&[i]).get_integer() as usize]);
						//dbg!(&current);
					}

					//dbg!(&indices);
					//dbg!(&current);

					program.push(current);
				}));
			}
			'0'..='9' => {
				let mut number = String::new();
				number.push(c);
				while let Some('0'..='9') = chars.peek() {
					number.push(chars.next().unwrap());
				}
				tokens.push(Token::Integer(number.parse::<i32>().unwrap()));
			}
			'.' => {
				let mut number = String::new();
				number.push(c);
				while let Some('0'..='9') = chars.peek() {
					number.push(chars.next().unwrap());
				}
				tokens.push(Token::Real(number.parse::<f32>().unwrap()));
			}
			'\'' => {
				let mut character = String::new();
				character.push(chars.next().unwrap());
				if chars.next().unwrap() != '\'' {
					panic!("Expected closing '");
				}
				tokens.push(Token::Char(character.chars().next().unwrap()));
			}
			'(' => {
				//push everything inbetween the parenthesis as a program
				let mut program = String::new();
				let mut nb_parenthesis = 1;
				while let Some(c) = chars.next() {
					if c == '(' {
						nb_parenthesis += 1;
					} else if c == ')' {
						nb_parenthesis -= 1;
						if nb_parenthesis == 0 {
							break;
						}
					}
					program.push(c);
				}
				tokens.push(Token::String(program));
			}
			'"' => {
				//string token
				let mut string = String::new();
				//put all the chars in the string until we find a closing "
				while let Some(c) = chars.next() {
					if c == '"' {
						break;
					}
					string.push(c);
				}
				tokens.push(Token::String(string));
			}
			'N' => tokens.push(Token::Set(Set::Integer)),
			'R' => tokens.push(Token::Set(Set::Real)),
			'C' => tokens.push(Token::Set(Set::Char)),
			'+' => tokens.push(Token::Operator(|program| {
				let a = program.pop().unwrap();
				let b = program.pop().unwrap();
				let a = a.get_single_value();
				let b = b.get_single_value();
				program.push(NDArray::addition(a, b));
			})),
			'-' => tokens.push(Token::Operator(|program| {
				let a = program.pop().unwrap();
				let b = program.pop().unwrap();
				let a = a.get_single_value();
				let b = b.get_single_value();
				program.push(NDArray::substraction(a, b));
			})),
			//copy operator
			'©' => tokens.push(Token::Operator(|program| {
				let a = program.pop().unwrap();
				program.push(a.clone());
				program.push(a.clone());
			})),
			// create an ndarray
			'⊹' => tokens.push(Token::Operator(|program| {
				let nb_dims = program.pop().unwrap().get_integer();
				let mut shape = Vec::new();
				for _ in 0..nb_dims {
					let dim = program.pop().unwrap().get_integer();
					shape.push(dim as usize);
				}
				let array = NDArray::zeros(&shape);
				program.push(array);
			})),
			// FIXME : c'est juste un test celui la
			'¤' => tokens.push(Token::RealTimeMacro(|program| {
				let stack = &program.stack;
				let a = stack.peek().unwrap();
				let a = a.get_integer();
				let code = if a == 8 { "9 9 +" } else { "8 8 +" };
				return String::from(code);
			})),
			'⋱' => tokens.push(Token::RealTimeMacro(|program| {
				return String::from("©2⊹=1[]2[]");
			})),
			'[' => {
				let next_char = chars.peek().unwrap();
				if *next_char == ']' {
					tokens.push(Token::Operator(|program| {
						let i = program.pop().unwrap();
						let i = i.get_integer() as usize;
						let result = program.indices_current[i];
						let result = NDArray::SingleValue(SingleValue::Integer(result as i32));
						program.push(result);
					}));
				}
			}
			'i' => {
				//ifelse token
				//take the 5 next chars
				let mut ifelse = String::new();
				for _ in 0..5 {
					ifelse.push(chars.next().unwrap());
				}
				if ifelse == "felse" {
					tokens.push(Token::Operator(|program| {
						let condition = program.pop().unwrap();
						let condition = condition.get_integer();
						let if_code = program.pop().unwrap();
						let else_code = program.pop().unwrap();

						let bloc_to_execute = if condition == 0 { else_code } else { if_code };
						//turn the bloc into a program
						let mut program_string = String::new();
						if let NDArray::NDArray { shape, inner } = bloc_to_execute {
							for x in inner {
								program_string.push(x.get_char());
							}
						} else {
							program_string.push(bloc_to_execute.get_char());
						}
						let mut other_program = Program::subprogram(&program_string, program);
						other_program.execute();
						let result = other_program.pop().unwrap();
						program.push(result);
					}));
				}
			}
			// while token
			'w' => {
				//while token
				//take the 5 next chars
				let mut while_ = String::new();
				for _ in 0..5 {
					while_.push(chars.next().unwrap());
				}
				if while_ == "hile " {
					tokens.push(Token::Operator(|program| {
						let condition = program.pop().unwrap();
						let mut condition = condition.get_integer();
						let while_code = program.pop().unwrap();

						let mut program_string = String::new();
						if let NDArray::NDArray { shape, inner } = while_code {
							for x in inner {
								program_string.push(x.get_char());
							}
						} else {
							program_string.push(while_code.get_char());
						}

						while condition != 0 {
							let mut other_program = Program::subprogram(&program_string, program);
							other_program.execute();
							let result = other_program.pop().unwrap();
							program.push(result);
							let _condition = program.pop().unwrap();
							condition = _condition.get_integer();
						}
					}));
				}
			}
			// modulo
			'%' => {
				tokens.push(Token::Operator(|program| {
					let a = program.pop().unwrap();
					let b = program.pop().unwrap();
					let a = a.get_integer();
					let b = b.get_integer();
					dbg!(a);
					dbg!(b);
					program.push(NDArray::SingleValue(SingleValue::Integer(a % b)));
				}));
			}
			// map operator
			'≝' => {}
			_ => {}
		}
	}
	tokens
}

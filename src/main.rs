mod ndarray;
mod program;
mod stack;
mod token;

use program::Program;

fn main() {
	let mut program = Program::new(
		"6 5 +
			© 2 ⊹",
	);
	/*let mut program = Program::new(
		"⊹ 2©
		+ 5 %",
	);*/
	println!("{:?}", program);
	println!("code : {}", program.get_code());
	program.execute();
	println!("{:?}", program.stack);
	println!("--- Result ---");
	println!("{:?}", program.get_result());

	/*let mut program = Program::new(
		"
		1 3 == \"5 5 +\" \"9 9 +\" ifelse
	",
	);*/

	let mut program = Program::new(
		"
			(5 5 +) 
			
			(9 9 +)
			3 1 ==
			ifelse
	",
	);
	// else block
	// if block
	// condition
	// keyword

	let tokens = token::tokenize(&program.get_code());
	println!(
		"{:?}",
		tokens.into_iter().rev().collect::<Vec<token::Token>>()
	);
	program.execute();
	println!("{:?}", program.stack);
	println!("--- IFELSE ---");
	println!("{:?}", program.get_result());


	/*let tokens = token::tokenize(&program.get_code());
	println!(
		"{:?}",
		tokens.into_iter().rev().collect::<Vec<token::Token>>()
	);
	program.execute();
	println!("{:?}", program.stack);
	println!("--- Result ---");
	println!("{:?}", program.stack.pop().unwrap());*/

	/*let mut program = Program::new("0[]");
	println!("{:?}", program);
	println!("{}", program.get_code());
	program.execute();
	println!("{:?}", program.stack);

	let mut program = Program::load("src/identity.truc");
	println!("{:?}", program);
	println!("{}", program.get_code());*/

	//17⋇^v≝1₁+⓪%0≣¤+2≣ -> je ???
 
	// is equal to 2?
	// apply the + operation
	// on an accumulator
	// of an array
	// where each element x is
	// is top_of_stack % (index of x + 1) == 0
	/*let mut is_prime = Program::new(
		"
		== 2
		\"+\"
		¤

		7 © 1 ⊹
	",
	); // 7 is the prime we want to test
	*/

	println!("--- SIX ---");

	let mut six = Program::new("5 5 5 5 6");
	six.execute();
	println!("end of program : {:?}\n", six.get_result());

	println!("--- ARRAY OF SEVENS ---");
	let mut array_of_sevens = Program::new("5 1 ⊹ (7) =");
	array_of_sevens.execute();
	println!("{:?}", array_of_sevens.stack);
	println!("end of program : {}", array_of_sevens.stack.pop().unwrap());

	println!("--- IS PRIME ---");
	let mut is_prime = Program::new("7 © 1 ⊹ (1[] 1 + % 0 ==) ="); // 7 is the prime we want to test
	is_prime.execute();
	println!("{:?}", is_prime.stack);
	println!("end of program : {}", is_prime.get_result());

	println!("--- DOUBLE SEVENS ---");
	let mut double_sevens = Program::new("5 5 2 ⊹ (2 1 ⊹ (7) =) =");
	double_sevens.execute();
	println!("{:?}", double_sevens.stack);
	println!("end of program : {}", double_sevens.stack.pop().unwrap());

	println!("--- IDENTITY MATRIX ---");
	let mut indentity_matrix = Program::new("5 5 2 ⊹ (5 1 ⊹ (1[]2[]==) =) =");
	indentity_matrix.execute();
	println!("{:?}", indentity_matrix.stack);
	println!("end of program : {}", indentity_matrix.stack.pop().unwrap());

	println!("--- CONVERSIONS ---");
	let mut conversions = Program::new("2 R § 5 R § /");
	conversions.execute();
	println!("{:?}", conversions.stack);
	println!("end of program : {}", conversions.stack.pop().unwrap());

	println!("--- WHILE ---");
	let mut while_loop = Program::new(
		"
			(5 5 +) 
			
			(9 9 +)
			3 1 ==
			while ",
	);
	while_loop.execute();
	println!("{:?}", while_loop.stack);
	println!("--- Result ---");

	println!("--- SELF ---");
	let mut self_program = Program::new("5 &");
	self_program.execute();
	println!("{:?}", self_program.stack);
	println!("end of program : {}", self_program.stack.pop().unwrap());
	let mut self_program = Program::new("5 1 ⊹ © (&) =");
	self_program.execute();
	println!("{:?}", self_program.stack);
	println!("end of program : {}", self_program.stack.pop().unwrap());

	println!("--- RESHAPE ---");
	let mut reshape = Program::new("1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 4 4 2 ^");
	reshape.execute();
	println!("{:?}", reshape.stack);
	println!("end of program : {:?}", reshape.stack.pop().unwrap());

	println!("--- ADD INDEX ---");
	let mut add_index = Program::new("5 1 ⊹ (1[] 1 +) = 1 3 2 1 ^ @");
	add_index.execute();
	println!("{:?}", add_index.stack);
	println!("end of program : {:?}", add_index.stack.pop().unwrap());
	let mut add_index = Program::new("5 5 2 ⊹ (5 1 ⊹ (1[]2[]+) =) = 1 3 2 3 1 ^ @");
	add_index.execute();
	println!("{:?}", add_index.stack);
	println!("end of program : {:?}", add_index.stack.pop().unwrap());

	println!("--- SUM OF ARRAY ---");
	let mut sum_of_array = Program::new("5 1 ⊹ (1[] 1 +) = ©© 0 1 (+) ¨");
	sum_of_array.execute();
	println!("{:?}", sum_of_array.stack);
	println!("end of program : {:?}", sum_of_array.stack.pop().unwrap());

    println!("--- SUM OF ARRAY ---");
    let mut sum_of_array = Program::new("5 1 ⊹ (1[] 1 +) = ©© 0 1 (+) ¨");
    sum_of_array.execute();
    println!("{:?}", sum_of_array.stack);
    println!("end of program : {:?}", sum_of_array.stack.pop().unwrap());

	println!("--- rjejh ---");
	let mut program = Program::new(
		"
		620 371 -
		");
	program.execute();
	println!("{:?}", program.stack);
	println!("--- Result ---");
	println!("{:?}", program.get_result());

}

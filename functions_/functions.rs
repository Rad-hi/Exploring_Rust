fn main(){
	println!("From main: Calling another function!");
	other('U', 69);

	const A: u8 = 2;
	const B: u8 = 25;

	println!("Sum of A: {}, and B: {} => A + B = {}", A, B, sum(A, B));

	println!("Printing {}", print_88());

	println!("Printing A: {} + 1 => {} ", A, print_var_plus_1(A));
}

fn other(key:char, val:u8){
	println!("From other: Param value --> {} : {}", key, val);
}

// An expression is turned into a statement when it's terminated with a semicolon
// Otherwise, it's a statement, and it doesn't return anything 
fn sum(a:u8, b:u8) -> u8{
	a+b
}

fn print_88() -> u8{
	88
}

fn print_var_plus_1(var:u8) -> u8{
	var + 1
	// var + 1; WRONG !
}
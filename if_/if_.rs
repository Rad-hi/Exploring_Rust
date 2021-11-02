fn main(){
	let num = 4;

	// Condition
	if num % 5 == 0 {
		println!("Divisible by 5");
	} else if num % 2 == 0 {
		println!("Divisible by 2");
	} else {
		println!("Else");
	}

	// Conditional assignment
	let parity = if num % 2 == 0 {"EVEN"} else {"ODD"};
	println!("Num is {}", parity);
}
/*
	The single quote (') before the [outer, inner] values signals that that's a label
	for the loop so that breaking loops could be done from anywhere.
	---
	Any loop could return a value, and that value could be used by other parts
	of the code, and this could be achieved by putting the value after the break
	BUT, we need to terminate the outermost loop with a semicolon (;) in order to
	turn it into a statement and let the value be assgined to the variable (laval)
	---
	We can use a while loop just as in C
	---
	We can loop through elements in a "list" with for
 */

fn main() {

	println!("/* ------------------- LOOP -------------------- */");
	let mut counter = 1;
	let yo = 'outer: loop{
             	counter += 20;
             	'inner: loop {
             		counter -= 5;
             		if counter == 0 {
             			println!("\tDone");
             			break 'inner;
             		} else {
             			counter += 5 - counter;
             		}
             	}

             	if counter == 0{
             		break 'outer "\tQUITTING";
             	}
             };

	println!("{}", yo);

	println!("/* ------------------- WHILE ------------------- */");
	counter += 3;
	while counter != 0 {
		counter -= 1;
		println!("\t{}", counter);
	}

	println!("/* -------------------- FOR -------------------- */");
	let tab = [1, 15, 16, 95, 255];
	for element in tab {
		print!("\t{}", element);
	}
	println!("");

	for i in (2 .. 7).rev() {
		print!("\t{}", i);
	}
	println!("");
}
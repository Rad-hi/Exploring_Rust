// ------------------------------------------------------------------
// src: https://doc.rust-lang.org/book/ __ Chapter 2, page 21 --> 41.
// ------------------------------------------------------------------

use std::io;                // Standard lib
use std::cmp::Ordering;
use rand::Rng;              // From crates.io 

fn main(){
    println!("--------------------------------");
    println!("--- NUMBER GUESSER, STARTED! ---"); // Any function that ends with a '!' is a macro
    println!("--------------------------------");
    
    let secret_number = rand::thread_rng().gen_range(1..101); // 1 to 100 ([1, 101[ <-> (1..=100) )
    
    println!("Please enter your guess (between 1 - 100)");

    loop{            
        
        let mut guess = String::new(); // Variables are immutable by default, so mut makes them mutable

        // Read from the standard input
        io::stdin()
            .read_line(&mut guess) // A mutable reference to the variable guess
            .expect("Failed to read line"); // Error handling
    
    // ------------------------------------------------------------------------------------------
    // We need to cast the String input into an integer while handling the non-number entry error
    // ------------------------------------------------------------------------------------------

    // First way --> breaking on non-number
        //let guess: u32 = guess.trim().parse().expect("Please insert a number > 1 !");
        
    // Second way --> ignoring non-number (any error)
        let guess: u32 = match guess.trim().parse(){ // (C similarity) Cast the input string to a an uint32_t number
            Ok(num) => num,
            Err(_) =>{
                println!("--- Please make sure to insert a number! ---");
                continue;
            }
        };
        
        /* match is similar to a switch in C */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("< Too small >"),
            Ordering::Greater => println!("< Too big >"),
            Ordering::Equal =>{
                println!("< You win >");
                break;
            }
        }; // END __ match
    } // END __ loop
} // END __ main

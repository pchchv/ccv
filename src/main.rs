use std::io;

use std::io;

fn main() {
    println!("You can enter 'exit' anytime to quit.");

    let mut cc_number = String::new();

    loop {
        println!("Please enter a CC number to validate: ")

        io::stdin().read_line(&mut cc_number).expect("Failed to read line");
    }
}

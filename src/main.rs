use std::io;

fn main() {
    println!("You can enter 'exit' anytime to quit.");

    loop {
        println!("Please enter a CC number to validate: ");

        let mut cc_number = String::new();

        io::stdin()
            .read_line(&mut cc_number)
            .expect("Failed to read line");

        cc_number = cc_number.trim().to_string();

        if cc_number == "exit" {
            break;
        }

        println!("{cc_number}");
    }
}

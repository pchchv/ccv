use std::io;

fn validator(cc_number: Vec<u8>) -> bool {
    let mut len = cc_number.len();
    let mut double_even_sum: u64 = 0;

    while len > 0 {
        len -= 1;

        // Double each even number, starting from the right
        // If the result is a two-digit number, add both digits to get one single-digit number
        // Add all resulting elements to 'doubleEvenSum'

        // Add each odd element, starting from the right, without changing, to 'double_even_sum'

        let mut digit = cc_number[len] as u64;

        if len % 2 == 0 {
            digit *= 2;
            
            if digit > 9 {
                digit = (digit / 10) + (digit % 10);
            }
        }
        double_even_sum += digit;
    }

    // If 'double_even_sum' is a multiple of 10, this is a valid credit card number

    if double_even_sum % 10 == 0 {
        return true;
    } else {
        return false;
    }
}

fn to_vec(ccn: &str) -> Vec<u8> {
    let ccn = ccn.split("");
    let mut cc_number: Vec<u8> = Vec::new();

    for i in ccn {
        let i: u8 = match i.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        cc_number.push(i);
    }

    cc_number
}

fn main() {
    println!("You can enter 'exit' anytime to quit.");

    loop {
        println!("Please enter a CC number to validate: ");

        let mut cc_number = String::new();

        io::stdin()
            .read_line(&mut cc_number)
            .expect("Failed to read line");

        let cc_number = cc_number.trim();

        if cc_number == "exit" {
            break;
        }

        if cc_number.parse::<u128>().is_ok() {
            let cc_number = to_vec(cc_number);
            if validator(cc_number) {
                println!("Valid")
            } else {
                println!("Invalid")
            }
        } else {
            eprintln!("Enter the correct number (numbers only)")
        }
    }
}

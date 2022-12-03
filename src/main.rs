use std::io;

fn validator(cc_number: Vec<u8>) -> bool {
    let len = cc_number.len() as i8;
    let mut double_even_sum: u64 = 0;
    let mut i: i8 = len - 2;

    // Double every second digit, starting from the right
    // If the result is a two-digit number,
    // add both digits to get one single-digit number
    // Finally, sum all the answers to obtain 'doubleEvenSum'

    while i >= 0 {
        let mut digit = cc_number[i as usize] * 2;

        if digit > 9 {
            digit = (digit / 10) + (digit % 10);
        }

        double_even_sum += digit as u64;

        i -= 2;
    }

    // Add every odd digit from the right to the 'double_even_sum'

    let mut i = len - 1;

    while i >= 0 {
        double_even_sum += cc_number[i as usize] as u64;

        i -= 2;
    }

    // Check if 'double_even_sum' is a multiple of 10
    // If yes, it is a valid credit card number

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

use std::io;

fn validator(cc_number: Vec<u8>) -> bool {
    println!("{:?}", cc_number);

    false
}

fn to_vec(ccn: &str) -> Vec<u8> {
    let ccn = ccn.split("");
    let mut cc_number: Vec<u8> = Vec::new();

    for i in ccn {
        println!("{i}");
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
            println!("{}", validator(cc_number))
        } else {
            eprintln!("Enter the correct number (numbers only)")
        }
    }
}

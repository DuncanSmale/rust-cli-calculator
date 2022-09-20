use std::{
    io::stdout,
    io::{stdin, Write},
};

fn main() {
    let mut total: i32 = 0;
    loop {
        let mut input_num = String::new();
        print!("Please enter a number to be added: ");
        stdout().flush().expect("Failed to flush");
        stdin()
            .read_line(&mut input_num)
            .expect("Unable to read user input");
        if input_num.trim() == "" || input_num.trim().to_uppercase() == "EXIT" {
            break;
        }
        let num_array = input_num.split(",").map(|el| {
            remove_whitespace(el)
                .parse::<i32>()
                .expect("Number to be valid")
        });
        num_array.for_each(|el| total += el);
    }
    println!("Your total is: {}", total);
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

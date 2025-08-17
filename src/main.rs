use std::{error::Error, io};

fn main() -> Result<(), std::io::Error> {
    let mut number: i32 = 0;

    println!(
        "<-- FizzBuzz! -->
        Starting from 1, keep counting with the computer until failure.
        If the next number is a multiple of 3, say Fizz instead.
        If the next number is a multiple of 5, say Buzz.
        If your number is a multiple of 3 and 5, say FizzBuzz!
        
        {number}"
    );

    loop {
        // User Input
        let mut input: String = String::new();

        let answer = check_answer(number.checked_add(1).unwrap());
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_ascii_lowercase();

        // Lowercase last so we don't check if we can lowercase the '!'. Optimizations are Critical here.
        if !(input == answer.strip_suffix("!").unwrap().to_lowercase()) {
            println!("Gotcha! It was {}!", answer);
            break;
        };

        // Checked_add doesn't assign. Fix tmr
        if let None = number.checked_add(1) {
            println!("I\'m tired... I give up... You win...");
            break;
        }
        println!("{}", check_answer(number))
    }

    println!("Press Enter to Exit");
    // Instead of returning Ok(()) explicitly, we can force it to here with the .and() method.
    io::stdin().read_line(&mut String::new()).and(Ok(()))
}

fn check_answer(num: i32) -> String {
    match (num % 3 == 0, num % 5 == 0) {
        (false, false) => num.to_string(),
        (true, false) => String::from("Fizz"),
        (false, true) => String::from("Buzz"),
        (true, true) => String::from(format!("FizzBuzz!")),
    }
}

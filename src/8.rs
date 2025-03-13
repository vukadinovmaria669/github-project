use std::cmp::Ordering;

fn main() {
    let mut guess = String::new();

    println!("Guess the number between 1 and 10!");

    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => (),
            Err(error) => println!("Error: {}", error),
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Invalid input: {}", error);
                continue;
            }
        };

        match guess.cmp(&5) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}

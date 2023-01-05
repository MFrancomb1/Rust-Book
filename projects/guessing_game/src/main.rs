//bring the input/output library into scope
//io library comes from the standard library(std)
use std::io;
//the Ordering type is an enum with variants Less, Greater, and Equal.
use std::cmp::Ordering;
//the Rng trait defines methods that random number generators inplement
use rand::Rng;

//main function, entry point of the program
fn main() {
    //println! is a macro that prints a string
    println!("Guess the number! ");

    //call rand::thread_rng function gives a random number generator local to the current thread of execution and seeded by the OS.
    //call gen_range method on the generator (defined by the Rng trait) that generates a random number in the range specified.
    //range expression: start..=end is inclusive on upper and lower bounds, so 1..=100 is a number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //infinite loop
    loop {
        println!("Please input your guess: ");

        //create new string variable named guess
        //mut makes the variable mutable
        //String::new is a function that returns a new String instance
        // (::) indicates that new is an associated function of String type.
        let mut guess = String::new();

        //call the stdin function from the io library
        //stdin() returns and instance of std::io::Stdin, i type that represents a handle to standard input for the terminal.
        io::stdin()
            //call the read_line method on the standard input handle
            //passing &mut guess as the string to store the input in
            // & indicates that the argument is a reference, multiple parts of the code can access one piece of data without copying it to memory multiple times
            //returns a Result value (enum)
            .read_line(&mut guess)
            //if Result returned is Err, program will crash and siplay error message
            .expect("Failed to read line");

        //convert guess (String) to a unsigned 32 bit integer
        //Shadowing allows for the reuse of guess variable name rather tham renaming
        //trim method to eliminate whitespace(the enter key in the terminal adds a newline to the string)
        //parse method on strigns converts a string to another type (u32)
        // match expression for error handling, will ignore inputs that cannot be converted to u32
        let guess: u32 = match guess.trim().parse() {
            //parse will return Ok value that contains a number if guess is converted successfully
            Ok(num) => num,
            //if number cannot be converted, parse will return Err value
            // _ is a catchall value
            // continue tells the program to go to the next iteration of the loop
            Err(_) => continue,
        };

        // {guess} is a placeholder for the guess variable
        println!("You guesesed: {guess}");

        //cmp method compares two values (guess and secret_number)
        //returns a variant of the Ordering enum.
        // match expression decides what to do based on the variant of Ordering.
        match guess.cmp(&secret_number) {
            //match expressoion made of 'arms' consisting of a pattern to match against and code to run if it matches.
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                //break exits the loop
                break;
            }
        }
    }
}

fn main() {
    
    let number = 1;

    //if expressions
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 2 or 3");
    }

    //if expression in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


    //LOOPS
    
    let mut x = 1;
    loop {

        println!("x: {x}");

        if x >= 5 {
            break
        } else {
            x = x + 1
        }
    }

    // returning a value from a loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    //loop labels begins with a single quote
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
     

    //conditional loops with while

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("Liftoff!");

    // for loops
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}")
    }

    //for loops in range
    //.rev() reverses the range (like i--)
    for number in (1..4).rev() {
        println!("{number!}");
    }
    println!("LIFTOFF!");
}
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //constants can be used in the global scope
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    //shadowing 
    //bind y to the value 10
    let y = 10;
    //(shadow) create a new variable by repeating let y =
    let y = y + 10;

    {
        //(shadow) create a new variable y for the current scope
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    //shadowing allows for changing the data type.
    let spaces = "    ";
    let spaces = spaces.len();
    println!("SPACES: {spaces}");

    //DATA TYPES

    //floating point types are f32 and f64 (all signed)
    let float = 2.2; // f64
    let g: f32 = 3.0; // f32
    println!("Floating point numbers: {float}, {g}");

    //boolean type (1 byte)
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("Boolean type: {t}, {f}");

    // char type
    let ch = 'h';
    let d: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{ch}, {d}, {heart_eyed_cat}");

    //COMPOUND DATA TYPES

    //tuples
    //fixed length. each position in the tuple has a type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //using pattern matching to destructure a tuple
    // using a pattern with let turns the tuple into 3 seperate variables
    let (_a, b, _c) = tup;
    println!("The value of b is: {b}");

    //or access a tuple element directly using a period (.) followed by the index of the value
    let first = tup.0;
    println!("{first}");


    //arrays
    //fixed length. all elements must have the same type
    //data allocated on the stack rather than the heap
    let arr = [1, 2, 3, 4, 5];
    //write the type an length using square brackets
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    //initialize an array to contain the same value for each element([value; length])
    let arr3 = [1; 5];
    println!("{} {} {}", arr[0], arr2[0], arr3[0]);
}

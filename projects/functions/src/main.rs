fn main() {
    println!("Hello, world!");

    let five = five();
    println!("The value of five is: {five}");

    another_function(5);

    print_labeled_measurement(5, 'h');

    //passing an the result of an expression into f
    println!("{}", f({
        let z = 1;
        z + 1
    }));

    let y = {

        //EXPRESSION: this block that evaluates to 4
        let x = 3;
        x + 1
    };

}

fn another_function(x: i32) {
    println!("the value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn f(x: i32) -> i32 {
    x + 1
}
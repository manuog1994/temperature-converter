use std::io;

fn main() {
    println!("Temperature Converter");

    let degree: [&str; 2] = ["[1] To Celsius", "[2] To Fahrenheit"];

    println!("Please select one option:");
    for element in degree  {

        println!("{element}");

    }

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed select");

    let input: usize = input
        .trim()
        .parse()
        .expect("The value entered not a number");

    println!("Please enter degrees to convert");

    let mut number: String = String::new();

    io::stdin() 
        .read_line(&mut number)
        .expect("Failed input");

    let number: f32 = number
        .trim()
        .parse()
        .expect("The value entered is not a number");


    if input == 1 {
        to_celsius(number);
    } else if input == 2 {
        to_fahrenheit(number);
    }
}

fn to_celsius(fahrenheit: f32) {
    let convert= (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{fahrenheit} Fahrenheit is: {convert} Celsius");
}

fn to_fahrenheit(celsius: f32) {
    let convert = (celsius * 9.0 / 5.0) + 32.0;

    println!("{celsius} Celsius is: {convert} Fahrenheit");
}
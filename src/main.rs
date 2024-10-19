use std::io;

fn main() {
    println!("Which operation would you like to use?");
    let mut input = String::new();
    error_checker(&mut input);

    let operator = input.trim();

    println!("What is the first number?");
    let mut input1 = String::new();
    error_checker(&mut input1);

    println!("What is the second number?");
    let mut input2 = String::new();
    error_checker(&mut input2);

    let input1_converted = convert_to_integer(&mut input1);
    let input2_converted = convert_to_integer(&mut input2);

    if operator == "+" {
        let answer = input1_converted + input2_converted;

        println!("The answer is: {answer}");
    }

    if operator == "-" {
        let answer = input1_converted - input2_converted;

        println!("The answer is: {answer}");
    }

    if operator == "*" {
        let answer = input1_converted * input2_converted;

        println!("The answer is: {answer}");
    }

    if operator == "/" {
        let answer = input1_converted / input2_converted;

        println!("The answer is: {answer}");
    }
}

fn error_checker(input_field: &mut String) {
    io::stdin()
        .read_line(input_field)
        .expect("Failed to read line");
}

fn convert_to_integer(input_field: &mut String) -> i32 {
    return input_field.trim().parse::<i32>().unwrap();
}

use std::env::{args, Args};

//For usage of parenthesis, use '(' or \( in terminal

fn operation(num_1: f32, num_2: f32, op: char) -> f32 {
    match op {
        '+' => num_1 + num_2,
        '-' => num_1 - num_2,
        '*' | 'x' | 'X' => num_1 * num_2,
        '/' => num_1 / num_2,
        _ => panic!("Invalid operator!!"),
    }
}

fn is_symbol(argument: &String) -> bool {
    if argument == "+" || argument == "-" || argument == "/" || argument == "x" || argument == "*" {
        return true;
    }
    false
}

fn do_operation(operators:&mut Vec<char>, numbers:&mut Vec<f32>){
    let num1: f32 = numbers.pop().unwrap();
    let num2: f32 = numbers.pop().unwrap();
    let operator: char = operators.pop().unwrap();

    //num2 first because it's the firstest number in comparrision
    let result = operation(num2, num1, operator);
    numbers.push(result);
}

fn main() {
    let mut inputs: Args = args(); //Iterator of inputs received by the user, the first input is the target
    let mut operators = Vec::<char>::new(); //Using as a stack of mathematical operators
    let mut numbers = Vec::<f32>::new(); //Using as a stack of float numbers
    let mut is_in_parenthesis: bool = false; //flag to determine if it's in a parenthesis

    inputs.next(); //Skips de first argument, that is the target

    for argument in inputs {
        if is_in_parenthesis && argument == ")" {
            while !(operators.last().copied().unwrap() == '(') {
                do_operation(&mut operators, &mut numbers);
            }
            operators.pop(); // pop the '(' from stack
            is_in_parenthesis = false;
        } 
        else {
            if is_symbol(&argument) {
                operators.push(argument.chars().next().unwrap());
            } else if argument == "(" {
                is_in_parenthesis = true;
                operators.push(argument.chars().next().unwrap());
            } else {
                numbers.push(argument.parse::<f32>().unwrap());
            }
        }
        if !is_in_parenthesis && numbers.len() > 1 && operators.len() > 0 {
            do_operation(&mut operators, &mut numbers);
        }
    }

    println!("Result - {:?}", numbers);
}

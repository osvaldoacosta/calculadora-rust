use std::env::{args, Args};

fn operation(num_1: f32, num_2: f32, op: char) -> f32 {
    match op {
        '+' => num_1 + num_2,
        '-' => num_1 - num_2,
        '*' | 'x' | 'X' => num_1 * num_2,
        '/' => num_1 / num_2,
        _ => panic!("Invalid operator!!"),
    }
}

fn is_operation(argument:&String) -> bool{
    if argument == "+" || argument == "-" || argument == "/" || argument == "x" || argument == "*"|| argument == "X"{
        return true
    }
    false
}

fn output(num_1: f32, num_2: f32, op: char) -> String {
    format!(
        "{} {} {} = {}",
        num_1,
        op,
        num_2,
        operation(num_1, num_2, op)
    )
}



fn main() {
    let mut str:Args = args();
    let mut operators = Vec::<char>::new();
    let mut numbers = Vec::<f32>::new();
    str.next(); //Skips de first argument that is the target

    for argument in str {
        if is_operation(&argument){
            operators.push(argument.chars().next().unwrap());
        }
        
        else {
            numbers.push(argument.parse::<f32>().unwrap());
        }

        
        if numbers.len()>1 && operators.len() > 0 {
            let num1:f32 = numbers.pop().unwrap();
            let num2:f32 = numbers.pop().unwrap();
            let operator:char = operators.pop().unwrap(); 
            //num2 first because it's the firstest number in comparrision
            let result = operation(num2, num1, operator);
            numbers.push(result);
        }
            

    }
    
    println!("Result - {:?}", numbers.pop().unwrap());
    



}

// args function returns args struct
use std::env::{args, Args};

fn main() {
    // mut keyword assigns this variable to be mutable
    let mut args: Args = args();

    // .upwrap() gets rid of the Some() wrapper
    // but if you use it on a value that doesnt exist, rust will panic
    // note that this also pops the value off of the array (hence using the zeroth element every time)
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    // parse numbers from string values
    // the : f32 lets the parse() method know to take the string value and parse it into an f32
    // let first_num: f32 = first.parse().unwrap();

    // or you can use turbofish syntax to do the same thing
    let first_num = first.parse::<f32>().unwrap();
    let second_num = second.parse::<f32>().unwrap();

    let result = operate(operator, first_num, second_num);

    // :? tells the compiler how to format args to be printed
    // println!("{:?}", first);
    // println!("{:?}", operator);
    // println!("{:?}", second);

    println!("{:?}", output(first_num, operator, second_num, result));

}

fn operate(operator: char, first_num: f32, second_num: f32) -> f32 {

    // char is single quotes, double quotes are of type string
    // so "+" will throw a type error here
    
    // this is kind of ugly and there is a better way to do it
    // if operator == '+' {
    //     first_num + second_num
    // } 
    // else if operator == '-' {
    //     first_num - second_num
    // }
    // else if operator == '/' {
    //     first_num / second_num
    // }
    // else if operator == '*' {
    //     first_num * second_num
    // }
    // else is always required!
    // else {
    //     // rust allows implicit returns with no semicolon
    //     0.0
    // }

    // we can use rust's expression matching syntax here instead

    // this is awesome!
    match operator {
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '/' => first_num / second_num,
        // handle multiple cases....
        '*' | 'x' | 'X' => first_num * second_num,
        // base case, 'else'...always required!
        _ => panic!("Invalid operator used.")
    }
}

fn output(first_num: f32, operator: char, second_num: f32, result: f32) ->  String {
    // format macro
    format!("{} {} {} = {}", first_num, operator, second_num, result)
}
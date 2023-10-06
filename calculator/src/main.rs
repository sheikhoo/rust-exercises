use std::env::{args,Args};

fn main() {
    let mut args: Args=args();

    let fisr_num: f32 = args.nth(1).unwrap().parse::<f32>().unwrap();
    let oprator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second_num: f32 = args.nth(0).unwrap().parse::<f32>().unwrap();
    let result: f32 = operate(oprator, fisr_num, second_num);

    println!("{} {} {} = {}",fisr_num,oprator,second_num,result);
}

fn operate(oprator: char,fisr_num: f32,second_num: f32) -> f32 {
    if oprator == '+' {
        fisr_num + second_num
    } else if oprator == '-' {
        fisr_num - second_num
    } else if oprator == '/' {
        fisr_num / second_num
    } else if oprator == '*' {
        fisr_num * second_num
    } else {
        0.0
    }
}
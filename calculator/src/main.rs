use std::env::{args,Args};

fn main() {
    let mut args: Args=args();

    let fisr_num: f32 = args.nth(1).unwrap().parse::<f32>().unwrap();
    let oprator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second_num: f32 = args.nth(0).unwrap().parse::<f32>().unwrap();
    let result: f32 = operate(oprator, fisr_num, second_num);

    println!("{}",output(fisr_num,oprator,second_num,result));
}

fn operate(oprator: char,fisr_num: f32,second_num: f32) -> f32 {
    match oprator {
        '+' => fisr_num + second_num,
        '-' => fisr_num - second_num,
        '/' => fisr_num / second_num,
        '*' | 'x' | 'X' => fisr_num * second_num,
        _   => panic!("Invalid operator used.")
    }
}

fn output(fisr_num: f32, oprator: char, second_num: f32,result: f32) -> String {
    format!("{} {} {} = {}",fisr_num,oprator,second_num,result)
}
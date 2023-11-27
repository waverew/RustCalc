use std::io::{stdin};
fn main() {
    println!("Hello, please enter first number");
    let mut inp1 = String::new();
    stdin().read_line(&mut inp1);
    let parsInp1: i32 = inp1.trim().parse().unwrap();
    println!("Please enter second number");
    let mut inp2 = String::new();
    stdin().read_line(&mut inp2);
    let parsInp2: i32 = inp2.trim().parse().unwrap();
    println!("Good, very good, now enter what you want to do with them");
    let mut com = String::new();
    stdin().read_line(&mut com);
    let parseCom = com.trim();
    let plus = String::from("+");
    let minus = String::from("-");
    let mult = String::from("*");
    let div = String::from("/"); 
    if parseCom == plus {
        println!("{}", calculate(parsInp1, parsInp2, Command::Add));
    } else if parseCom == minus {
        println!("{}", calculate(parsInp1, parsInp2, Command::Sub));
    } else if parseCom == mult {
        println!("{}", calculate(parsInp1, parsInp2, Command::Multi));
    } else if parseCom == div {
        println!("{}", calculate(parsInp1, parsInp2, Command::Divide));
    }
}

pub enum Command {
    Add,
    Multi,
    Sub,
    Divide
}

fn calculate(inp1: i32, inp2: i32, command: Command) -> i32 {
    match command {
        Command::Add => return inp1 + inp2,
        Command::Multi => return inp1 * inp2,
        Command::Sub => return inp1 - inp2,
        Command::Divide => return inp1/inp2
    }
}

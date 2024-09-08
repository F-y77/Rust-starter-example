use regex::Regex;
use std::io::{self, Write};

fn main() {
    println!("你好！我是你的聊天机器人。你可以问我任何问题。");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.to_lowercase() == "退出" {
            println!("再见！");
            break;
        }

        let response = match_input(&input);
        println!("{}", response);
    }
}

fn match_input(input: &str) -> &'static str {
    let re = Regex::new(r"(你好|您好|嗨)").unwrap();
    if re.is_match(input) {
        return "你好！很高兴见到你。";
    }

    let re = Regex::new(r"(天气|气候)").unwrap();
    if re.is_match(input) {
        return "今天的天气很好！";
    }

    let re = Regex::new(r"(时间|几点了)").unwrap();
    if re.is_match(input) {
        return "现在是下午3点。";
    }

    "我不太明白你的问题，请再说一遍。"
}

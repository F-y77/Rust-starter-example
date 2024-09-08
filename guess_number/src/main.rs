use rand::Rng;
use std::io;

fn main() {
    // 打印欢迎信息和游戏说明
    println!("欢迎来到猜数字游戏！");
    println!("系统已经生成了一个1到100之间的随机数字。");

    // 生成1到100之间的随机数作为秘密数字
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // 进入游戏循环，直到玩家猜对数字
    loop {
        println!("请输入你的猜测：");

        let mut guess = String::new();

        // 读取玩家输入的猜测数字
        io::stdin()
            .read_line(&mut guess)
            .expect("读取输入失败");

        // 将输入的字符串转换为数字，如果转换失败则提示玩家重新输入
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的数字！");
                continue;
            }
        };

        println!("你猜测的数字是: {}", guess);

        // 比较玩家猜测的数字与秘密数字，并给出相应的提示
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("太小了！"),
            std::cmp::Ordering::Greater => println!("太大了！"),
            std::cmp::Ordering::Equal => {
                println!("恭喜你，猜对了！");
                break;
            }
        }
    }
}
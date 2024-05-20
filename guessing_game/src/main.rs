use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, get {}.", value);
        }

        Self { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("猜数！");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("神秘数字是: {}", secret_number);

    loop {
        println!("猜测一个数！");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        // shadow
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);

        println!("你猜的数是: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

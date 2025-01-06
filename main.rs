use std::io;
use std::cmp::Ordering;
use rand::Rng;

enum IpprAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ippr {
    kind: IpprAddress,
    name: String,
}

impl Ippr {
    fn call(&self){
        println!("I'm calling from {:?}", self.name);
    }
}

fn main() {
    let home = Ippr {
        kind: IpprAddress::V4(127, 0, 0, 7),
        name: String::from("Home"),
    };
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is: {}", secret_number);



        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}

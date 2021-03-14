use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Tebak Angka / Guess the number!");
    //generate random number with rand library
    //lower inclusif and upper ekslusif
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please input yout number: ");

        //make mutable string variable
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to process / input the number");
        /** 
         *casting guess to int32 and using match 
          expressing to repeat the program when error and
          take the value if success to parse
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Yeyy you got it!");
                break;
            }
        }
    }
 
}

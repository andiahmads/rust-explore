use rand::Rng;
use std::cmp::Ordering;
use std::io; // method perbadingan bertipe enum

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret_number is: {}", secret_number);

    
    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        println!("your guess: {}", guess);

     

        // lakukan teknik shadowing yg berfungsi untuk convert string ke INT
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // value cocok
            Err(_) => continue, //tidak melakukan apapun
        };

        println!("your guessed {}", guess);


        /*
            match dapat digunakan untuk menjalankan kode secara kondisional.
            dan me return nilai enum
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("to small!"),
            Ordering::Greater => println!("to Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

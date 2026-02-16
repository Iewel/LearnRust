use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut times = 0;
    
    loop{
	println!("Please input your guess.");
	let mut guess = String::new();

	io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
	let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue,
	};
    
	println!("You guessed : {}", guess);
	
	match guess.cmp(&secret_number){
	    Ordering::Less => {
		println!("Too small!");
		times = times+1;
	    },
	    Ordering::Greater => {
		println!("Too big!");
		times = times + 1;
	    },
	    Ordering::Equal => {
		println!("Correct!!!");
		times = times + 1;
		println!("The scret number is : {}, and you guess {} times.", secret_number, times);
		break;
	    }
	}
    }
}

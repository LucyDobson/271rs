use rand::Rng;
use std::io;
use colored::Colorize;

fn main() {
	let words: [&str;6] = ["acorn", "mouth", "siege", "ratio", "filly", "guide"];
	let number = rand::rng().random_range(1..=6);

	println!("First word: {}", words[number]);
        println!();
	
	let mut g = ["str","str","str","str","str","str"];
	
	for i in 1..6{
		let mut guess = String::new();

		io::stdin()
               		.read_line(&mut guess)
                 	.expect("Failed to read line ");
	
		let guess_str = guess.clone();
        	g[i] = guess_str.as_str();
        	println!("First in array: {}", g[0]);

        	print!("|");

        	for c in guess.chars() {
        	print!(" {} |" , c);
        	}



	}

}

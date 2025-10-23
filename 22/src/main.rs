use rand::Rng;
use std::io;
use colored::Colorize;

fn main() {
	let words: [&str;6] = ["acorn", "mouth", "siege", "ratio", "filly", "guide"];
	let number = rand::rng().random_range(0..=6);


        let secret = words[number];
	//println!("First word: {}", secret);
        println!();
	
	let mut g = ["str".to_string(),"str".to_string(),"str".to_string(),"str".to_string(),"str".to_string(),"str".to_string()];
        print!("\x1B[2J");
        let mut guess = String::new();
        for i in 1..6{
	    println!("Guess the word: ");	
            
                let mut guess = String::new();

		io::stdin()
               		.read_line(&mut guess)
                 	.expect("Failed to read line ");
	
		let guess_str = guess.clone();
        	g[i] = guess_str;
        	//println!("First in array: {}", g[i].to_string().blue());
		
        	
                //println!("Secret: {}", secret);
                //println!("Guess: {}", guess);

                println!();

		//print!("|");

                for (i, g_char)in guess.chars().enumerate(){
        	    if let Some(s_char) = secret.chars().nth(i){

                        if g_char == s_char{
                            print!("{}", g_char.to_string().green());
                        }   
                        else if secret.contains(g_char){
                            print!("{}", g_char.to_string().yellow());
                        }
                        else{
                            print!("{}", g_char.to_string().red());
                        }   
                    }   

                }
                
                if guess.to_string().trim() == secret.to_string(){
                break;
                }
                
                println!();
	}
        println!("The word was: {}", secret.blue());
        if guess.to_string().trim() == secret.to_string(){
            println!("{}","You win!".green());
            }
            else{
                println!("{}", "You lose.".red());
            }


}

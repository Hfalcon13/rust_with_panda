use std::{io::{self, Write}};


pub fn length_of_last_word(s: String) -> i32
{
    s.trim().split(' ').collect::<Vec<_>>().last().unwrap().len() as i32
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();  // Ensure the prompt is displayed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()  // Trim the newline character and return the input
}

fn gen_targil_by_score(score: i32) -> Option<f64>
{
    match score {
        n if n < 10 => 
        {
            
        }
        _ => 
        {
            println!("bro ended the game");
            None
        }
    }
}

pub fn math_game_main()
{
    println!("test")
}





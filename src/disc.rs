use std::{io::{self, Write}, ops::Range};

use rand::Rng;


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

trait InRange {
	fn in_range(self, range: Range<i32>) -> bool;
}

impl InRange for i32
{
	fn in_range(self, range: Range<i32>) -> bool {
		range.contains(&self)
	}
}

fn gen_targil_by_score(score: i32) -> Option<i32>
{
	let mut rng = rand::thread_rng();
	match score {
		n if n < 10 => 
		{
			let a = rng.gen_range(0..10);
			let b = rng.gen_range(0..10);
			println!("{} + {}", a, b);
			Some(a + b)
		}
		n if n.in_range(10..20) =>
		{
			let a = rng.gen_range(0..20);
			let b = rng.gen_range(0..20);
			println!("{} + {}", a, b);
			Some(a + b)
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
	let mut score = 0;
	loop {
		let result = gen_targil_by_score(score);
		if result.is_none()
		{
			break;
		}
		else
		{
			let user_result = read_line().parse::<i32>().unwrap();
			if result.unwrap() == user_result
			{
				score += 1;
			}
			else
			{
				score -= 1;
			}
		}
	}
}





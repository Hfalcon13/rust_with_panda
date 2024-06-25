use std::{io::{self, Write}, ops::{Add, Sub, Mul, Div}};

use num::FromPrimitive;
use num_derive::FromPrimitive;
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

// trait InRange {
// 	fn in_range(self, range: Range<i32>) -> bool;
// }

// impl InRange for i32
// {
// 	fn in_range(self, range: Range<i32>) -> bool {
// 		range.contains(&self)
// 	}
// }

#[derive(FromPrimitive, PartialEq, Eq)]
enum Op {
	Add,
	Sub,
	Mul,
	Div,
}
impl Op
{
	pub fn to_char_and_fn(self) -> (char, fn(i32, i32) -> i32)
	{
		match self {
			Op::Add => ('+', i32::add as fn(i32, i32) -> i32),
			Op::Sub => ('-', i32::sub as fn(i32, i32) -> i32),
			Op::Mul => ('*', i32::mul as fn(i32, i32) -> i32),
			Op::Div => ('/', i32::div as fn(i32, i32) -> i32),
		}
	}
}

fn gen_targil_by_score(score: i32) -> Option<i32>
{
	let mut rng = rand::thread_rng();

	let arena = score / 10;

	match arena {
		0 => 
		{
			let a = rng.gen_range(0..10);
			let b = rng.gen_range(0..10);
			println!("{} + {}", a, b);
			Some(a + b)
		}
		1 =>
		{
			let a = rng.gen_range(0..100);
			let b = rng.gen_range(0..100);
			println!("{} + {}", a, b);
			Some(a + b)
		}
		2 =>
		{
			let a = rng.gen_range(0..10);
			let b = rng.gen_range(0..10);
			println!("{} - {}", a, b);
			Some(a - b)
		}
		3 =>
		{
			let a = rng.gen_range(0..100);
			let b = rng.gen_range(0..100);
			println!("{} - {}", a, b);
			Some(a - b)
		}
		4 => 
		{
			let op: Op = FromPrimitive::from_u32(rng.gen_range(0..4)).unwrap();
			let a = rng.gen_range(0..10);
			let b = rng.gen_range(0..10);

			if op == Op::Div && b == 0
			{
				return gen_targil_by_score(score);
			}

			let pair = op.to_char_and_fn();

			println!("{} {} {}", a, pair.0, b);

			Some(pair.1(a, b))
		}
		5 =>
		{
			let a = rng.gen_range(-7..=7);
			let b = rng.gen_range(-7..=7);
			let c = rng.gen_range(-7..=7);

			if f64::floor(-b as f64 / (2 * a) as f64) != -b as f64 / (2 * a) as f64 ||
				a == 0 
			{
				return gen_targil_by_score(score);
			}

			println!("{}x^2 + {}x + {}\nfind the extrema point", a, b, c);

			Some(-b / (2 * a))
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
	let mut score = 50;
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





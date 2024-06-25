use std::{collections::HashMap, sync::Arc};


struct StateBuilder<'a>
{
	alphabet: Arc<[char]>,
	transitions: HashMap<char, &'a State<'a>>,
	is_starting: bool,
	is_ending: bool,
}

impl<'a> StateBuilder<'a>
{
	pub fn new(alphabet: Box<[char]>, is_starting: bool, is_ending: bool) -> Self
	{
		Self
		{
			alphabet: alphabet.into(),
			transitions: HashMap::new(),
			is_starting,
			is_ending,
		}
	}
	pub fn add_transition(&mut self, state: &'a State<'a>, char: char)
	{
		self.transitions.insert(char, state);
	}
	pub fn build(self) -> State<'a>
	{
		State
		{
			alphabet: self.alphabet,
			transitions: self.transitions,
			is_starting: self.is_starting,
			is_ending: self.is_ending,
		}
	}
}

struct State<'a>
{
	alphabet: Arc<[char]>,
	transitions: HashMap<char, &'a State<'a>>,
	is_starting: bool,
	is_ending: bool,
}



struct Dfa<'a>
{
	alphabet: Arc<[char]>,
	states: Arc<[State<'a>]>,
}

// impl<'a> Dfa<'a>
// {
// 	fn get_starting(&self) -> &'a State<'a>
// 	{
// 		//self.states.iter().filter(|state| state.is_starting)
// 	}
// 	pub fn is_in_language(&self, str: &str) -> bool
// 	{
// 		// TODO: Implement this function
// 	}
// }
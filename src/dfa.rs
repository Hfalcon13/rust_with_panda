use std::{collections::HashMap, sync::Arc};


pub struct StateBuilder<'a>
{
	alphabet: Arc<[char]>,
	transitions: HashMap<char, &'a State<'a>>,
	is_starting: bool,
	is_ending: bool,
}

impl<'a> StateBuilder<'a>
{
	pub fn new(alphabet: Arc<[char]>, is_starting: bool, is_ending: bool) -> Self
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
	pub fn build(&self) -> State<'a>
	{
		State
		{
			alphabet: self.alphabet.clone(),
			transitions: self.transitions.clone(),
			is_starting: self.is_starting,
			is_ending: self.is_ending,
		}
	}
}

pub struct State<'a>
{
	alphabet: Arc<[char]>,
	transitions: HashMap<char, &'a State<'a>>,
	is_starting: bool,
	is_ending: bool,
}

impl<'a> State<'a>
{
	fn transition(&self, char: char) -> &'a Self
	{
		*self.transitions.get(&char).unwrap()
	}
}


pub struct Dfa<'a>
{
	alphabet: Arc<[char]>,
	states: Arc<[State<'a>]>,
}

impl<'a> Dfa<'a>
{
	pub fn new(alphabet: Arc<[char]>, states: &[State<'a>]) -> Dfa<'a>
	{
		let boxed_slice: Box<[State]> = states.iter()
        .map(|state| State
			{
				alphabet: state.alphabet.clone(),
				transitions: state.transitions.clone(),
				is_starting: state.is_starting,
				is_ending: state.is_ending,
			})
        .collect::<Vec<_>>()
        .into_boxed_slice();;

    	// Convert the boxed slice to Arc<[State]>
    	let arc_states: Arc<[State]> = Arc::from(boxed_slice);
		Self
		{
			alphabet,
			states: arc_states,
		}
	}

	fn get_starting(&'a self) -> &'a State<'a>
	{
		assert_eq!(self.states.iter().filter(|x| x.is_starting).collect::<Vec<_>>().len(), 1);
		assert!(self.states.iter().find(|x| x.is_starting).is_some());
		self.states.iter().find(|x| x.is_starting).unwrap()
	}
	pub fn is_in_language(&self, str: &str) -> bool
	{
		self.internal_is_in_language(str, self.get_starting())
	}
	fn internal_is_in_language(&self, str: &str, state: &'a State<'a>) -> bool
	{
		if str.len() == 0
		{
			state.is_ending
		}
		else
		{
			let (c, s) = str.chars().next().map(|first| (first, &str[first.len_utf8()..])).unwrap();
			self.internal_is_in_language(s, state.transition(c))
		}
	}
}
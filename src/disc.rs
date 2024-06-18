
pub fn length_of_last_word(s: String) -> i32 {
    s.trim().split(' ').collect::<Vec<_>>().last().unwrap().len() as i32
}







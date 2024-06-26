
//use disc::math_game_main;



pub mod disc;
pub mod dfa;


// fn run_ml(buf: &[u8], arg: u64) -> u64
// {
// 		use virtual_memory::VirtualMemory;
//     let mut memory = VirtualMemory::new(buf.len()).expect("failed to allocate rwx memory");
//     memory.copy_from_slice(buf);

//     let f: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(memory.as_ptr()) };

//     return f(arg);
// }

fn main() {

	//let code = [0x6A, 0x01, 0x58, 0x48, 0x0F, 0xAF, 0xC1, 0x48, 0xFF, 0xC9, 0x75, 0xF7, 0xC3];

    //let res = run_ml(&code, 9);

    //println!("{}", res);

    //math_game_main()
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use dfa::{Dfa, StateBuilder};

    use super::*;

    #[test]
    fn test_automatone() {

		let alphabet = Arc::new(['a', 'b']);

        let sb1 = StateBuilder::new(alphabet.clone(), true, false);
        let sb2 = StateBuilder::new(alphabet.clone(), false, true);

        let s1 = sb1.build();
        let s2 = sb2.build();

        let mut sb1_again = StateBuilder::new(alphabet.clone(), true, false);
        sb1_again.add_transition(&s2, 'a');
        sb1_again.add_transition(&s1, 'b');

        let mut sb2_again = StateBuilder::new(alphabet.clone(), false, true);
        sb2_again.add_transition(&s1, 'a');
        sb2_again.add_transition(&s2, 'b');

        let dfa = Dfa::new(alphabet, &[s1]);
		
		let strs = ["aabababaabb", "bbabbaaab", "aabaabba"];

		assert!(dfa.is_in_language(strs[0]));
		assert!(dfa.is_in_language(strs[1]));
		assert!(dfa.is_in_language(strs[2]));

    }
}






















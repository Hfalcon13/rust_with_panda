
//use disc::math_game_main;
use virtual_memory::VirtualMemory;



pub mod disc;


fn run_ml(buf: &[u8], arg: u64) -> u64
{
    let mut memory = VirtualMemory::new(buf.len()).expect("failed to allocate rwx memory");
    memory.copy_from_slice(buf);

    let f: extern "C" fn(u64) -> u64 = unsafe { std::mem::transmute(memory.as_ptr()) };

    return f(arg);
}

fn main() {

	let code = [0x6A, 0x01, 0x58, 0x48, 0x0F, 0xAF, 0xC1, 0x48, 0xFF, 0xC9, 0x75, 0xF7, 0xC3];

    let res = run_ml(&code, 9);

    println!("{}", res);

    //math_game_main()
}
























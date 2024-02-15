
use std::arch::asm;

fn main() {
    let t = 100;
    let t_ptr: *const usize = &t; // if you comment out this...
    // ...and uncomment the line below. The program will fail.   
    // let t_ptr = 99999999999999 as *const usize;
    let x = dereference(t_ptr);

    println!("{}", x);
}

#[allow(unused)]
fn dereference_x86_64(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe { 
        asm!("mov {0}, [{1}]", out(reg) res, in(reg) ptr)
    };
    res
}

fn dereference(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe {
        // Using `ldr` instruction for ARM64 to load the value from a pointer into a register.
        // Note: Inline assembly syntax and registers might need adjustments based on
        // specific use cases and compiler versions.
        asm!("ldr {res}, [{ptr}]", res = out(reg) res, ptr = in(reg) ptr)
    };
    res
}


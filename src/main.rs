#[repr(C)]
#[derive(Clone, Copy)]
enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
    COUNT,
}

fn main() {
    let memory = [0u16; 1 << 16 - 1];
    let registers = [Register::R0; Register::COUNT as usize];
    println!("Hello, world!");
}

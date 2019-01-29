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

#[repr(C)]
#[derive(Clone, Copy)]
enum Opcode {
    BRANCH = 0,
    ADD,
    LOAD,
    STORE,
    JUMPR,
    AND,
    LOADR,
    STORER,
    RTI,
    NOT,
    LOADI,
    STOREI,
    JUMP,
    RES,
    LEA,
    TRAP,
}

impl From<u16> for Opcode {
    fn from(op: u16) -> Self {
        match op {
            0 => Opcode::BRANCH,
            1 => Opcode::ADD,
            2 => Opcode::LOAD,
            3 => Opcode::STORE,
            4 => Opcode::JUMPR,
            5 => Opcode::AND,
            6 => Opcode::LOADR,
            7 => Opcode::STORER,
            8 => Opcode::RTI,
            9 => Opcode::NOT,
            10 => Opcode::LOADI,
            11 => Opcode::STOREI,
            12 => Opcode::JUMP,
            13 => Opcode::RES,
            14 => Opcode::LEA,
            15 => Opcode::TRAP,
            _ => panic!("Invalid opcode: {}", op),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
enum Flag {
    POSITIVE = 1 << 0,
    ZERO = 1 << 1,
    NEGATIVE = 1 << 2,
}

fn sign_extend(x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 == 1{
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}

fn get_flag(r: u16) -> Flag {
    if r == 0 { Flag::ZERO }
    else if r >> 15 == 1 { Flag::NEGATIVE }
    else { Flag::POSITIVE }
}

fn main() {
    let mut memory = [0u16; 1 << 16 - 1];
    let mut registers = [0u16; Register::COUNT as usize];

    let mut running = true;
    while running {
        let instr = memory[registers[Register::PC as usize] as usize];
        let op = instr >> 12;

        match op.into() {
            Opcode::ADD => {
                let r0 = (instr >> 9) & 0x7;
                let r1 = (instr >> 6) & 0x7;
                let imm_flag = (instr >> 5) & 0x1;

                if imm_flag == 1 {
                    let imm5 = sign_extend(instr & 0x1F, 5);
                    registers[r0 as usize] = registers[r1 as usize] + imm5;
                } else {
                    let r2 = instr & 0x7;
                    registers[r0 as usize] = registers[r1 as usize] + registers[r2 as usize];
                }

                registers[Register::COND as usize] = get_flag(registers[r0 as usize]) as u16;
            },
            Opcode::AND => {},
            Opcode::NOT => {},
            Opcode::BRANCH => {},
            Opcode::JUMP => {},
            Opcode::JUMPR => {},
            Opcode::LOAD => {},
            Opcode::LOADI => {},
            Opcode::LOADR => {},
            Opcode::LEA => {},
            Opcode::STORE => {},
            Opcode::STOREI => {},
            Opcode::STORER => {},
            Opcode::TRAP => {},
            Opcode::RTI | Opcode::RES => {},
        }
    }
    println!("Hello, world!");
}

use std::io::Read;

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
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

#[repr(C)]
#[derive(Clone, Copy)]
enum Trap {
    GETC = 0x20,    // get character from keyboard
    OUT = 0x21,     // output a character
    PUTS = 0x22,    // output a word string
    IN = 0x23,      // input a string
    PUTSP = 0x24,   // output a byte string
    HALT = 0x25,    // halt the program
}

impl From<u16> for Trap {
    fn from(trap: u16) -> Self {
        match trap {
            0x20 => Trap::GETC,
            0x21 => Trap::OUT,
            0x22 => Trap::PUTS,
            0x23 => Trap::IN,
            0x24 => Trap::PUTSP,
            0x25 => Trap::HALT,
            _ => panic!("Invalid trap code: {}", trap),
        }
    }
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

fn update_flag(registers: &mut [u16; Register::COUNT as usize], r: u16) {
    registers[Register::COND as usize] = get_flag(registers[r as usize]) as u16;
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

                update_flag(&mut registers, r0);
            },
            Opcode::AND => {
                let r0 = (instr >> 9) & 0x7;
                let r1 = (instr >> 6) & 0x7;
                let imm_flag = (instr >> 5) & 0x1;

                if imm_flag == 1 {
                    let imm5 = sign_extend(instr & 0x1F, 5);
                    registers[r0 as usize] = registers[r1 as usize] & imm5;
                } else {
                    let r2 = instr & 0x7;
                    registers[r0 as usize] = registers[r1 as usize] & registers[r2 as usize];
                }

                update_flag(&mut registers, r0);
            },
            Opcode::NOT => {
                let r0 = (instr >> 9) & 0x7;
                let r1 = (instr >> 6) & 0x7;

                registers[r0 as usize] = !registers[r1 as usize];

                update_flag(&mut registers, r0);
            },
            Opcode::BRANCH => {
                let pc_offset = sign_extend(instr & 0x1FF, 9);
                if ((instr >> 9) & registers[Register::COND as usize] & 0x7) != 0 {
                    registers[Register::PC as usize] += pc_offset;
                }
            },
            Opcode::JUMP => {
                let r0 = (instr >> 6) & 0x7;
                registers[Register::PC as usize] = registers[r0 as usize];
            },
            Opcode::JUMPR => {
                registers[Register::R7 as usize] = registers[Register::PC as usize];
                let pc_offset = sign_extend(instr & 0x7FF, 11);
                if ((instr >> 11) & 1) != 0 {
                    registers[Register::PC as usize] += pc_offset;
                } else {
                    let r0 = (instr >> 6) & 0x7;
                    registers[Register::PC as usize] = registers[r0 as usize];
                }
            },
            Opcode::LOAD => {
                let r0 = (instr >> 9) & 0x7;
                let pc_offset = sign_extend(instr & 0x1FF, 9);
                registers[r0 as usize] = memory[registers[Register::PC as usize] as usize + pc_offset as usize];
                update_flag(&mut registers, r0);
            },
            Opcode::LOADI => {
                let r0 = (instr >> 9) & 0x7;
                let pc_offset = sign_extend(instr & 0x1FF, 9);
                registers[r0 as usize] = memory[memory[registers[Register::PC as usize] as usize + pc_offset as usize] as usize];
                update_flag(&mut registers, r0);
            },
            Opcode::LOADR => {
                let r0 = (instr >> 9) & 0x7;
                let base = (instr >> 6) & 0x7;
                let offset = sign_extend(instr & 0x3F, 6);
                registers[r0 as usize] = memory[registers[base as usize] as usize + offset as usize];
                update_flag(&mut registers, r0);
            },
            Opcode::LEA => {
                let r0 = (instr >> 9) & 0x7;
                let pc_offset = sign_extend(instr & 0x1FF, 9);
                registers[r0 as usize] = registers[Register::PC as usize] + pc_offset;
                update_flag(&mut registers, r0);
            },
            Opcode::STORE => {
                let r0 = (instr >> 9) & 0x7;
                let pc_offset = sign_extend(instr & 0x1FF, 9);
                memory[registers[Register::PC as usize] as usize + pc_offset as usize] = registers[r0 as usize];
            },
            Opcode::STOREI => {
                let r0 = (instr >> 9) & 0x7;
                let pc_offset = sign_extend(instr & 0x1FF, 9);
                memory[memory[registers[Register::PC as usize] as usize + pc_offset as usize] as usize] = registers[r0 as usize];
            },
            Opcode::STORER => {
                let r0 = (instr >> 9) & 0x7;
                let base = (instr >> 6) & 0x7;
                let offset = sign_extend(instr & 0x3F, 6);
                memory[registers[base as usize] as usize + offset as usize] = registers[r0 as usize];
            },
            Opcode::TRAP => {
                match (instr & 0xFF).into() {
                    Trap::GETC => {
                        registers[Register::R0 as usize] = std::io::stdin().bytes().next().unwrap().unwrap() as u16;
                    },
                    Trap::OUT => {
                        print!("{}", registers[Register::R0 as usize] as u8 as char);
                    },
                    Trap::PUTS => {
                        let mut i = registers[Register::R0 as usize] as usize;
                        while memory[i] != 0 {
                            print!("{}", memory[i] as u8 as char);
                            i += 1;
                        }
                    },
                    Trap::IN => {
                        print!("Enter a character: ");
                        registers[Register::R0 as usize] = std::io::stdin().bytes().next().unwrap().unwrap() as u16;
                    },
                    Trap::PUTSP => {
                        let mut i = registers[Register::R0 as usize] as usize;
                        while memory[i] != 0 {
                            print!("{}", (memory[i] & 0xFF) as u8 as char);
                            let ch = (memory[i] >> 8) as u8;
                            if ch != 0 {
                                print!("{}", ch as char);
                            }
                            i += 1;
                        }
                    },
                    Trap::HALT => {
                        println!("HALT");
                        running = false;
                    },
                }
            },
            Opcode::RTI | Opcode::RES => { panic!("Illegal Opcode {}", op) },
        }
    }
    println!("Hello, world!");
}

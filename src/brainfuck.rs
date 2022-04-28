use std::io::{self, Write};

#[derive(Debug)]
pub enum EvalError {
    Error
}

pub struct BF {
    mem: [u8; 30000],
    memptr: u16,
    stack: Vec<usize>
}

impl BF {
    pub fn new() -> Self {
        Self { mem: [0; 30000], memptr: 0, stack: Vec::new() }
    }

    pub fn eval(&mut self, code: &str) -> Result<(), EvalError> {
        let mut pc = 0;
        let prog_bytes: Vec<u8> = code.bytes().collect();
        let prog_len = code.len();
        
        while pc < prog_len {
            let mut jumped = false; // Reset jump flag
            let instr = prog_bytes[pc]; // Fetch instruction
            match instr {
                b'+' => { self.mem[self.memptr as usize] += 1; },
                b'-' => { self.mem[self.memptr as usize] -= 1; },
                b'<' => {
                    if self.memptr == 0 {
                        self.memptr = 29999;
                    } else {
                        self.memptr -= 1;
                    }
                },
                b'>' => {
                    if self.memptr == 29999 {
                        self.memptr = 0;
                    } else {
                        self.memptr += 1;
                    }
                }
                b'[' => { self.stack.push(pc); },
                b']' => {
                    let loop_pc = match self.stack.last() {
                        Some(v) => *v,
                        None => return Err(EvalError::Error)
                    };

                    if self.mem[self.memptr as usize] == 0 {
                        self.stack.pop().unwrap(); // Pop last loop pc
                    } else {
                        pc = loop_pc + 1;
                        jumped = true;
                    }
                },
                b'.' => { print!("{}", self.mem[self.memptr as usize] as char); io::stdout().flush().unwrap() }
                _ => {}
            }
            if !jumped {
                pc += 1;
            }
        }
        Ok(())
    }
}

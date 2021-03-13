use super::instruction::Instruction;
use std::io;
#[derive(Debug)]
pub struct Parser {
    pub instructions: Vec<Instruction>,
}
impl Parser {
    pub fn new<T>(f: &mut T) -> io::Result<Parser>
    where
        T: io::Read,
    {
        let mut buffer = Vec::new();
        let mut v: Vec<Instruction> = Vec::new();
        let mut s: Vec<u8> = Vec::new();
        let mut in_comment: bool = false;
        f.read_to_end(&mut buffer)?;
        let mut i = 0;
        loop {
            match buffer.get(i) {
                Some(ch) => match in_comment {
                    true => match ch {
                        b'\n' => {
                            in_comment = false;
                        }
                        _ => {
                            i += 1;
                        }
                    },
                    false => {
                        match ch {
                            b'\n' => {
                                match s.len() {
                                    0 => {}
                                    _ => {
                                        v.push(Instruction::from(&s).unwrap());
                                    }
                                }
                                s = Vec::new();
                            }
                            b'/' => match buffer.get(i + 1) {
                                Some(b'/') => {
                                    in_comment = true;
                                }
                                Some(b'\r') => {}
                                _ => {
                                    s.push(*ch);
                                }
                            },
                            b'\r' | b' ' => {}
                            _ => {
                                s.push(*ch);
                            }
                        }
                        i += 1;
                    }
                },

                None => break,
            }
        }
        Ok(Parser { instructions: v })
    }
}

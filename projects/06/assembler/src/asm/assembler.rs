use super::instruction::{Instruction, Symbol};
use super::opcode::Opcode;
use super::parser::Parser;
use super::symbol_table::SymbolTable;
use std::fmt::Write;
#[derive(Debug)]
pub struct Assembler {
    parser: Parser,
    st: SymbolTable,
}
impl Assembler {
    pub fn new<T>(f: &mut T) -> Assembler
    where
        T: std::io::Read,
    {
        let mut a = Assembler {
            parser: Parser::new(f).unwrap(),
            st: SymbolTable::new(),
        };
        a.build_symbol_table();
        a
    }
    fn build_symbol_table(&mut self) {
        let mut addr: i32 = 0;
        for ins in &self.parser.instructions {
            if let Instruction::L(Symbol::S(s)) = ins {
                if self.st.is_predefined(s) {
                    panic!("redifing predefined");
                }
                match self.st.contains(s) {
                    false => {
                        self.st.add_entry(s.clone(), addr);
                    }
                    true => {
                        panic!("redefining label: {}", s);
                    }
                };
            } else {
                addr += 1;
            }
        }
    }

    pub fn assemble<T>(&mut self, f: &mut T) -> Result<(), &'static str>
    where
        T: std::io::Write,
    {
        let mut addr: i32 = 16;
        for ins in &self.parser.instructions {
            match ins {
                Instruction::C(dest, comp, jump) => {
                    match Opcode::assemble_ctype(dest, comp, jump) {
                        Ok(opcode) => {
                            write!(f, "{:016b}\n", opcode);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Instruction::A(Symbol::I(num)) => match Opcode::assemble_atype(num.clone()) {
                    Ok(opcode) => {
                        write!(f, "{:016b}\n", opcode);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                },
                Instruction::A(Symbol::S(s)) => match self.st.contains(s) {
                    true => {
                        let addr = self.st.get_address(s);
                        match Opcode::assemble_atype(addr.clone()) {
                            Ok(opcode) => {
                                write!(f, "{:016b}\n", opcode);
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        };
                    }
                    false => match self.st.is_predefined(s) {
                        true => {
                            let addr = self.st.get_address_predefined(s);
                            match Opcode::assemble_atype(addr.clone()) {
                                Ok(opcode) => {
                                    write!(f, "{:016b}\n", opcode);
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            };
                        }

                        false => {
                            self.st.add_entry(s.clone(), addr);
                            match Opcode::assemble_atype(addr.clone()) {
                                Ok(opcode) => {
                                    write!(f, "{:016b}\n", opcode);
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            };
                            addr += 1;
                        }
                    },
                },

                _ => {}
            };
        }

        Ok(())
    }
}

use super::instruction::{Instruction, Symbol};

pub struct Opcode {}
impl Opcode {
    pub fn assemble_atype(addr: i32) -> Result<u16, &'static str> {
        if addr > (1 << 15) {
            return Err("addr out of range");
        }
        let mut opcode: u16 = 0;
        for i in 0..15 {
            if (addr & (1 << i)) != 0 {
                opcode ^= 1 << i;
            }
        }
        Ok(opcode)
    }
    pub fn assemble_ctype(
        dest: &String,
        comp: &String,
        jump: &String,
    ) -> Result<u16, &'static str> {
        let mut opcode: u16 = 57344;
        match Opcode::assemble_ctype_dest(dest) {
            Ok(opc) => {
                opcode |= opc;
            }
            Err(e) => {
                return Err(e);
            }
        };

        match Opcode::assemble_ctype_comp(comp) {
            Ok(opc) => {
                opcode |= opc;
            }
            Err(e) => {
                return Err(e);
            }
        };
        match Opcode::assemble_ctype_jump(jump) {
            Ok(opc) => {
                opcode |= opc;
            }
            Err(e) => {
                return Err(e);
            }
        };
        Ok(opcode)
    }
    fn assemble_ctype_comp(s: &String) -> Result<u16, &'static str> {
        match &s[..] {
            "0" => Ok(42 << 6),
            "1" => Ok(63 << 6),
            "-1" => Ok(58 << 6),
            "D" => Ok(12 << 6),
            "A" => Ok(48 << 6),
            "!D" => Ok(13 << 6),
            "!A" => Ok(49 << 6),
            "-D" => Ok(15 << 6),
            "-A" => Ok(51 << 6),
            "D+1" => Ok(31 << 6),
            "A+1" => Ok(55 << 6),
            "D-1" => Ok(14 << 6),
            "A-1" => Ok(50 << 6),
            "D+A" => Ok(2 << 6),
            "D-A" => Ok(19 << 6),
            "A-D" => Ok(7 << 6),
            "D&A" => Ok(0),
            "D|A" => Ok(21 << 6),
            "M" => Ok(112 << 6),
            "!M" => Ok(113 << 6),
            "M+1" => Ok(119 << 6),
            "M-1" => Ok(114 << 6),
            "D+M" => Ok(66 << 6),
            "D-M" => Ok(83 << 6),
            "M-D" => Ok(71 << 6),
            "D&M" => Ok(64 << 6),
            "D|M" => Ok(85 << 6),
            _ => Err("unkown comp operand"),
        }
    }

    fn assemble_ctype_dest(s: &String) -> Result<u16, &'static str> {
        match &s[..] {
            "" => Ok(0),
            "M" => Ok(1 << 3),
            "D" => Ok(2 << 3),
            "MD" => Ok(3 << 3),
            "A" => Ok(4 << 3),
            "AM" => Ok(5 << 3),
            "AD" => Ok(6 << 3),
            "AMD" => Ok(7 << 3),
            _ => Err("Unknown dest operand"),
        }
    }
    fn assemble_ctype_jump(s: &String) -> Result<u16, &'static str> {
        match &s[..] {
            "" => Ok(0),
            "JGT" => Ok(1),
            "JEQ" => Ok(2),
            "JGE" => Ok(3),
            "JLT" => Ok(4),
            "JNE" => Ok(5),
            "JLE" => Ok(6),
            "JMP" => Ok(7),
            _ => Err("Unknown jump operand"),
        }
    }
}

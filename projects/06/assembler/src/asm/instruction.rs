#[derive(Debug)]
pub enum Instruction {
    A(Symbol),
    C(String, String, String),
    L(Symbol),
}
impl Instruction {
    pub fn from(v: &Vec<u8>) -> Result<Instruction, &'static str> {
        match v[0] {
            b'@' => Instruction::decode_atype(v),
            b'(' => Instruction::decode_ltype(v),
            _ => Instruction::decode_ctype(v),
        }
    }
    fn decode_atype(v: &Vec<u8>) -> Result<Instruction, &'static str> {
        let s: String = String::from(std::str::from_utf8(&v[1..]).unwrap());
        match s.parse::<i32>() {
            Ok(num) => {
                return Ok(Instruction::A(Symbol::I(num)));
            }

            Err(_) => {
                return Ok(Instruction::A(Symbol::S(s)));
            }
        };
    }
    fn decode_ltype(v: &Vec<u8>) -> Result<Instruction, &'static str> {
        if v.last().cloned() != Some(b')') {
            return Err("syntax error for label type");
        }
        let s: String = String::from(std::str::from_utf8(&v[1..v.len() - 1]).unwrap());
        Ok(Instruction::L(Symbol::S(s)))
    }
    fn decode_ctype(v: &Vec<u8>) -> Result<Instruction, &'static str> {
        let (dest, comp, jump) = split(v).unwrap();
        Ok(Instruction::C(dest, comp, jump))
    }
}
#[derive(Debug)]
pub enum Symbol {
    S(String),
    I(i32),
}

fn split(v: &Vec<u8>) -> Result<(String, String, String), &'static str> {
    let mut dest: String = String::new();
    let mut comp: String = String::new();
    let mut jump: String = String::new();
    let mut done_dest = false;
    let mut done_comp = false;

    let mut i = 0;
    while i < v.len() {
        let mut j = i;
        while j < v.len() && v[j] != b'=' && v[j] != b';' {
            j += 1;
        }
        if j < v.len() {
            match v[j] {
                b'=' => {
                    dest = String::from(std::str::from_utf8(&v[i..j]).unwrap());
                    if dest.len() > 0 {
                        done_dest = true;
                    }
                }
                b';' => {
                    done_comp = true;
                    comp = String::from(std::str::from_utf8(&v[i..j]).unwrap());
                }
                _ => {
                    panic!("Unreachable");
                }
            }
        } else {
            match done_comp {
                false => {
                    match done_dest {
                        true => comp = String::from(std::str::from_utf8(&v[i..j]).unwrap()),
                        false => {
                            return Err("Syntax error both dest, jump fields missing");
                        }
                    };
                }
                true => jump = String::from(std::str::from_utf8(&v[i..j]).unwrap()),
            }
        }
        i = j + 1;
    }

    if (dest.len() == 0 && jump.len() == 0) || comp.len() == 0 {
        return Err("Syntax error");
    }

    return Ok((dest, comp, jump));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_csplit_1() {
        let s = "D=D-A";
        let dest = String::from("D");
        let comp = String::from("D-A");
        let jump = String::new();
        let (d, c, j) = split(&s.as_bytes().iter().cloned().collect()).unwrap();
        assert_eq!(dest, d);
        assert_eq!(comp, c);
        assert_eq!(jump, j);
    }
    #[test]
    fn test_csplit_2() {
        let s = "D=M";
        let dest = String::from("D");
        let comp = String::from("M");
        let jump = String::new();
        let (d, c, j) = split(&s.as_bytes().iter().cloned().collect()).unwrap();
        assert_eq!(dest, d);
        assert_eq!(comp, c);
        assert_eq!(jump, j);
    }
    #[test]
    fn test_csplit_3() {
        let s = "0;JMP";
        let dest = String::new();
        let comp = String::from("0");
        let jump = String::from("JMP");
        let (d, c, j) = split(&s.as_bytes().iter().cloned().collect()).unwrap();
        assert_eq!(dest, d);
        assert_eq!(comp, c);
        assert_eq!(jump, j);
    }
    #[test]
    fn test_csplit_4() {
        let s = "D;JGT";
        let dest = String::new();
        let comp = String::from("D");
        let jump = String::from("JGT");
        let (d, c, j) = split(&s.as_bytes().iter().cloned().collect()).unwrap();
        assert_eq!(dest, d);
        assert_eq!(comp, c);
        assert_eq!(jump, j);
    }
    #[test]
    fn test_csplit_5() {
        let s = "D=D-A;JEQ";
        let dest = String::from("D");
        let comp = String::from("D-A");
        let jump = String::from("JEQ");
        let (d, c, j) = split(&s.as_bytes().iter().cloned().collect()).unwrap();

        assert_eq!(dest, d);
        assert_eq!(comp, c);
        assert_eq!(jump, j);
    }
    #[test]
    #[should_panic]
    fn test_csplit_6() {
        let s = "";
        let (d, c, j) = split(&s.as_bytes().iter().cloned().collect()).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_csplit_7() {
        let s = "=D-A;";
        let (d, c, j) = split(&s.as_bytes().iter().cloned().collect()).unwrap();
    }
    #[test]
    #[should_panic]
    fn test_csplit_8() {
        let s = "D-A";
        let (d, c, j) = split(&s.as_bytes().iter().cloned().collect()).unwrap();
    }
}

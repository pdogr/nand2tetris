macro_rules! create_hashmap {
    ($($key: expr => $val: expr ),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert(String::from($key),$val); )*
        map
    }}
}

#[derive(Debug)]
pub struct SymbolTable {
    entries: std::collections::HashMap<String, i32>,
    predefined: std::collections::HashMap<String, i32>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut predefined: std::collections::HashMap<String, i32> = create_hashmap![
            "SP" => 0,
            "LCL" => 1,
            "ARG" => 2,
            "THIS" => 3,
            "THAT" => 4,
            "SCREEN" => 16384,
            "KBD" => 24676
        ];
        for i in 0..16 {
            let mut s = String::from("R");
            s.push_str(&i.to_string());
            predefined.insert(s, i);
        }

        SymbolTable {
            entries: std::collections::HashMap::new(),
            predefined,
        }
    }
    pub fn add_entry(&mut self, s: String, addr: i32) {
        self.entries.insert(s, addr);
    }
    pub fn is_predefined(&self, s: &String) -> bool {
        self.predefined.contains_key(s)
    }
    pub fn contains(&self, s: &String) -> bool {
        self.entries.contains_key(s)
    }
    pub fn get_address(&self, s: &String) -> i32 {
        if let Some(opt) = self.entries.get(s) {
            return opt.clone();
        } else {
            panic!("awd");
        }
    }
    pub fn get_address_predefined(&self, s: &String) -> i32 {
        if let Some(opt) = self.predefined.get(s) {
            return opt.clone();
        } else {
            panic!("awd");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_st_1() {
        let st = SymbolTable::new();
    }
}

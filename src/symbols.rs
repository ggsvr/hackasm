use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: HashMap<String, u16>,
    var_counter: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        let var_counter = 16;
        let mut symbols = HashMap::from([
            (String::from("SP"), 0),
            (String::from("LCL"), 1),
            (String::from("ARG"), 2),
            (String::from("THIS"), 3),
            (String::from("THAT"), 4),
            (String::from("SCREEN"), 0x4000),
            (String::from("KBD"), 0x6000),
        ]);
        for i in 0..16 {
            let s = format!("R{i}");
            symbols.insert(s, i);
        }
        Self {
            symbols,
            var_counter,
        }
    }

    pub fn add(&mut self, symbol: String, addr: u16) {
        self.symbols.insert(symbol, addr);
    }
    pub fn add_variable(&mut self, symbol: String) -> u16 {
        let out = self.var_counter;
        self.symbols.insert(symbol, self.var_counter);
        self.var_counter += 1;
        out
    }
    pub fn contains(&self, symbol: &str) -> bool {
        self.symbols.contains_key(symbol)
    }
    pub fn addr_of(&self, symbol: &str) -> Option<u16> {
        self.symbols.get(symbol).copied()
    }
}

use crate::instr::*;
use crate::symbols::SymbolTable;
use std::fmt::Write;

pub type Opcode = u16;

pub fn codegen_instructions(mut ins: Vec<Instr>) -> Vec<Opcode> {
    let mut symbol_table = SymbolTable::new();
    add_labels(&mut ins, &mut symbol_table);

    ins.iter()
        .filter_map(|i| instr_to_code(i, &mut symbol_table))
        .collect()
}
pub fn opcodes_to_string(opcodes: &[Opcode]) -> String {
    let mut out = String::with_capacity((opcodes.len() + 1) * 16);
    for op in opcodes.iter() {
        writeln!(&mut out, "{op:016b}").unwrap();
    }
    out
}

fn add_labels(instrs: &mut [Instr], st: &mut SymbolTable) {
    let mut ctr = 0;
    for ins in instrs.iter_mut() {
        // only update counter when it's a real instruction
        // we move the string out of the instruction because
        // it avoids another allocation and where not using it
        // later
        if let Instr::Label(l) = ins {
            st.add(std::mem::take(l), ctr);
        } else {
            ctr += 1;
        }
    }
}

fn instr_to_code(instr: &Instr, st: &mut SymbolTable) -> Option<Opcode> {
    match instr {
        Instr::A(l) => Some(load_to_code(l, st)),
        Instr::C { dest, comp, jump } => Some(c_to_code(*dest, *comp, *jump)),
        Instr::Label(_) => None,
    }
}

fn load_to_code(load: &Load, st: &mut SymbolTable) -> Opcode {
    match load {
        Load::Value(n) => *n,
        Load::Symbol(s) => match st.addr_of(s.as_str()) {
            Some(addr) => addr,
            None => st.add_variable(s.to_string()),
        },
    }
}
fn c_to_code(dest: CDest, comp: Comp, jump: Jump) -> Opcode {
    let j = jump as u16;
    let d = dest as u16;
    let c = comp as u16;
    j | (d << 3) | (c << 6) | 0b1110_0000_0000_0000
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_load(load: Load, st: &mut SymbolTable, expected: Opcode) {
        assert_eq!(load_to_code(&load, st), expected);
    }
    fn assert_load_symbol(name: &str, st: &mut SymbolTable, expected: Opcode) {
        assert_load(Load::Symbol(name.into()), st, expected);
    }
    fn assert_comp(dest: CDest, comp: Comp, jump: Jump, expected: Opcode) {
        assert_eq!(c_to_code(dest, comp, jump), expected);
    }
    #[test]
    fn load_codegen() {
        let mut st = SymbolTable::new();
        let st = &mut st;
        st.add("addr1".into(), 5);
        st.add("addr2".into(), 10);
        st.add("addr3".into(), 500);
        st.add("addr4".into(), 2);
        assert_load(Load::Value(0), st, 0);
        assert_load(Load::Value(1), st, 1);
        assert_load(Load::Value(200), st, 200);

        assert_load_symbol("var1", st, 16);
        assert_load_symbol("var2", st, 17);
        assert_load_symbol("var3", st, 18);
        assert_load_symbol("var4", st, 19);

        assert_load_symbol("addr1", st, 5);
        assert_load_symbol("addr2", st, 10);
        assert_load_symbol("addr3", st, 500);
        assert_load_symbol("addr4", st, 2);

        assert_load_symbol("SP", st, 0);
        assert_load_symbol("LCL", st, 1);
        assert_load_symbol("ARG", st, 2);
        assert_load_symbol("SCREEN", st, 0x4000);
    }
    #[test]
    fn comp_codegen() {
        assert_comp(CDest::None, Comp::Zero, Jump::None, 0b111_0_101010_000_000);
        assert_comp(CDest::A, Comp::One, Jump::JMP, 0b111_0_111111_100_111);
        assert_comp(CDest::AMD, Comp::M, Jump::JGT, 0b111_1_110000_111_001);
    }
}

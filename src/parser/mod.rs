mod comp;
use crate::instr::{CDest, Comp, Instr, Jump, Load};

#[allow(dead_code)]
pub fn parse_asm(source: &str) -> Result<Vec<Instr>, Vec<String>> {
    let processed = remove_comments(source);
    let mut instr = Vec::new();
    let mut errors = Vec::new();
    for word in processed.split_whitespace() {
        match parse_word(word) {
            Ok(i) => instr.push(i),
            Err(msg) => {
                errors.push(build_error(&processed, word, msg));
            }
        }
    }
    if errors.is_empty() {
        Ok(instr)
    } else {
        Err(errors)
    }
}

fn build_error(source: &str, word: &str, msg: String) -> String {
    let word_offset = word.as_ptr() as usize - source.as_ptr() as usize;
    let line_number = source[..word_offset].chars().filter(|c| *c == '\n').count() + 1;
    format!("error at line {line_number}: {msg}")
}

fn parse_word(word: &str) -> Result<Instr, String> {
    if word.starts_with('@') {
        parse_load(word)
    } else if word.starts_with('(') {
        parse_label(word)
    } else {
        parse_comp(word)
    }
}

fn parse_load(word: &str) -> Result<Instr, String> {
    assert_eq!(word.chars().next(), Some('@'));
    let operand = &word[1..];
    if operand.is_empty() {
        return Err("load instruction missing operand".into());
    }
    let load = match operand.parse::<u32>() {
        Ok(n) => {
            if check_15bit(n) {
                Load::Value(n as u16)
            } else {
                return Err(format!("load operand bigger than 15 bits: {n}"));
            }
        }
        Err(_) => {
            if check_ident(operand) {
                Load::Symbol(operand.to_string())
            } else {
                return Err(format!("invalid load operand identifier: {operand}"));
            }
        }
    };
    Ok(Instr::A(load))
}

fn parse_label(word: &str) -> Result<Instr, String> {
    assert_eq!(
        word.chars().next(),
        Some('('),
        "function shouldn't be called in this case"
    );
    if !word.ends_with(')') {
        return Err("unclosed label".into());
    }
    let symbol = &word[1..(word.len() - 1)];
    if symbol.is_empty() {
        return Err("empty label".into());
    }
    if !check_ident(symbol) {
        return Err(format!("invalid label identifier: {symbol}"));
    }
    Ok(Instr::Label(symbol.into()))
}

fn parse_comp(word: &str) -> Result<Instr, String> {
    let mut eq_split = word.split('=');
    let first = eq_split
        .next()
        .expect("shouldn't be called on empty string");

    // if rest is None, it means there isn't a '=' in the string,
    // so the destination is null and the string is the rest.
    // Else, we should parse the destination and return the rest.
    let (dest, rest_s) = match eq_split.next() {
        None => (CDest::None, first),
        Some(r) => (parse_dest(first)?, r),
    };

    let mut sc_split = rest_s.split(';');
    // the computation is the first half of the split.
    // error out if computation is empty.
    // let Some(comp) = sc_split.next() else {
    //     return Err("C-instruction must have a computation part".into());
    // };

    // parse the first half of the split as the computation
    let comp = parse_op(sc_split.next().unwrap())?;

    // now get the jump and parse it if there's anything after
    // the semicolon.
    let jump = match sc_split.next() {
        Some(j) => parse_jump(j)?,
        None => Jump::None,
    };
    Ok(Instr::C { dest, comp, jump })
}
fn parse_dest(word: &str) -> Result<CDest, String> {
    if word.is_empty() {
        return Err("empty destination before '='".into());
    }
    match comp::VALID_DESTS.get(word) {
        Some(dest) => Ok(*dest),
        None => Err(format!("invalid destination: {word}")),
    }
}
fn parse_op(word: &str) -> Result<Comp, String> {
    if word.is_empty() {
        return Err("empty computation".into());
    }
    match comp::VALID_OPS.get(word) {
        Some(c) => Ok(*c),
        None => Err(format!("invalid computation: {word}")),
    }
}
fn parse_jump(word: &str) -> Result<Jump, String> {
    if word.is_empty() {
        return Err("empty jump after ';'".into());
    }
    match comp::VALID_JMPS.get(word) {
        Some(j) => Ok(*j),
        None => Err(format!("invalid jump: {word}")),
    }
}
fn check_15bit(num: u32) -> bool {
    num < (1 << 15)
}

fn check_ident(word: &str) -> bool {
    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !is_ident_start(first) {
        return false;
    }
    for c in chars {
        if !is_ident_char(c) {
            return false;
        }
    }
    true
}

fn is_ident_char(c: char) -> bool {
    is_ident_start(c) || c.is_ascii_digit()
}
fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || c == '.' || c == '$' || c == ':'
}

fn remove_comments(mut source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    while let Some(i) = source.find("//") {
        // append everything until comment start
        out.push_str(&source[..i]);
        let comment_start = &source[i..];

        // find terminating newline
        let nl = comment_start.find('\n').unwrap_or(source[i..].len());
        let comment_len = comment_start[..nl].len();
        // fill it with blanks
        out.extend(std::iter::repeat(' ').take(comment_len));
        // and set source to its end
        source = &comment_start[nl..];
    }
    // now push the rest
    out.push_str(&source);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_error<T>(res: Result<T, String>) {
        assert!(matches!(res, Err(_)));
    }

    #[test]
    fn test_parse_load() {
        assert_eq!(parse_load("@abc"), Ok(Instr::A(Load::Symbol("abc".into()))));
        assert_eq!(parse_load("@123"), Ok(Instr::A(Load::Value(123))));
        assert_error(parse_load("@100000"));
        assert_error(parse_load("@"));
        assert_error(parse_load("@("));
        assert_error(parse_load("@;"));
    }
    #[test]
    fn test_parse_label() {
        assert_eq!(parse_label("(abc)"), Ok(Instr::Label("abc".into())));
        assert_eq!(
            parse_label("(abc:def:)"),
            Ok(Instr::Label("abc:def:".into()))
        );
        assert_error(parse_label("( abc )"));
        assert_error(parse_label("(123)"));
        assert_error(parse_label("(1abc)"));
        assert_error(parse_label("()"));
        assert_error(parse_label("( )"));
        assert_error(parse_label("(;abc)"));
        assert_error(parse_label("(abc"));
        assert_error(parse_label("( abc"));
    }

    fn assert_comp_ok(source: &str, dest: CDest, comp: Comp, jump: Jump) {
        assert_eq!(parse_comp(source), Ok(Instr::C { dest, comp, jump }));
    }
    fn assert_comp_err(source: &str) {
        assert!(matches!(parse_comp(source), Err(_)));
    }
    #[test]
    fn test_parse_comp() {
        assert_comp_ok("0", CDest::None, Comp::Zero, Jump::None);
        assert_comp_ok("A=1", CDest::A, Comp::One, Jump::None);
        assert_comp_err("A=");
        assert_comp_err("abc=");
        assert_comp_err("=0");
        assert_comp_err("=");

        assert_comp_ok("D;JMP", CDest::None, Comp::D, Jump::JMP);
        assert_comp_err(";");
        assert_comp_err("D;");
        assert_comp_err(";JMP");
        assert_comp_err(";abc");

        assert_comp_ok("AMD=D|A;JGE", CDest::AMD, Comp::DOrA, Jump::JGE);
    }

    #[test]
    fn test_remove_comments() {
        assert_eq!(
            remove_comments("abc//comment lalala \ndef"),
            "abc                 \ndef"
        );
        assert_eq!(
            remove_comments("abc //comment lalala \ndef"),
            "abc                  \ndef"
        );
        assert_eq!(
            remove_comments("abc//comment lalala \n"),
            "abc                 \n"
        );
        assert_eq!(
            remove_comments("abc//comment lalala"),
            "abc                "
        );
        assert_eq!(
            remove_comments("abc//comment //nested lalala \ndef"),
            "abc                          \ndef"
        );
    }
}

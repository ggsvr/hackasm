#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instr {
    A(Load),
    C { dest: CDest, comp: Comp, jump: Jump },
    Label(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Load {
    Symbol(String),
    Value(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CDest {
    None = 0,
    M = 1,
    D = 2,
    MD = 3,
    A = 4,
    AM = 5,
    AD = 6,
    AMD = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Jump {
    None = 0,
    JGT = 1,
    JEQ = 2,
    JGE = 3,
    JLT = 4,
    JNE = 5,
    JLE = 6,
    JMP = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Comp {
    Zero      = 0b0_101010,
    One       = 0b0_111111,
    MinusOne  = 0b0_111010,
    D         = 0b0_001100,
    A         = 0b0_110000,
    NotD      = 0b0_001101,
    NotA      = 0b0_110001,
    MinusD    = 0b0_001111,
    MinusA    = 0b0_110011,
    DPlusOne  = 0b0_011111,
    APlusOne  = 0b0_110111,
    DMinusOne = 0b0_001110,
    AMinusOne = 0b0_110010,
    DPlusA    = 0b0_000010,
    DMinusA   = 0b0_010011,
    AMinusD   = 0b0_000111,
    DAndA     = 0b0_000000,
    DOrA      = 0b0_010101,
    M         = 0b1_110000,
    NotM      = 0b1_110001,
    MinusM    = 0b1_110011,
    MPlusOne  = 0b1_110111,
    MMinusOne = 0b1_110010,
    DPlusM    = 0b1_000010,
    DMinusM   = 0b1_010011,
    MMinusD   = 0b1_000111,
    DAndM     = 0b1_000000,
    DOrM      = 0b1_010101,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn comp_variants() {
        use Comp::*;
        assert_eq!(Zero as u8, 0b0_101010);
        assert_eq!(One as u8, 0b0_111111);
        assert_eq!(MinusOne as u8, 0b0_111010);
        assert_eq!(D as u8, 0b0_001100);
        assert_eq!(A as u8, 0b0_110000);
        assert_eq!(NotD as u8, 0b0_001101);
        assert_eq!(NotA as u8, 0b0_110001);
        assert_eq!(MinusD as u8, 0b0_001111);
        assert_eq!(MinusA as u8, 0b0_110011);
        assert_eq!(DPlusOne as u8, 0b0_011111);
        assert_eq!(APlusOne as u8, 0b0_110111);
        assert_eq!(DMinusOne as u8, 0b0_001110);
        assert_eq!(AMinusOne as u8, 0b0_110010);
        assert_eq!(DPlusA as u8, 0b0_000010);
        assert_eq!(DMinusA as u8, 0b0_010011);
        assert_eq!(AMinusD as u8, 0b0_000111);
        assert_eq!(DAndA as u8, 0b0_000000);
        assert_eq!(DOrA as u8, 0b0_010101);
        assert_eq!(M as u8, 0b1_110000);
        assert_eq!(NotM as u8, 0b1_110001);
        assert_eq!(MinusM as u8, 0b1_110011);
        assert_eq!(MPlusOne as u8, 0b1_110111);
        assert_eq!(MMinusOne as u8, 0b1_110010);
        assert_eq!(DPlusM as u8, 0b1_000010);
        assert_eq!(DMinusM as u8, 0b1_010011);
        assert_eq!(MMinusD as u8, 0b1_000111);
        assert_eq!(DAndM as u8, 0b1_000000);
        assert_eq!(DOrM as u8, 0b1_010101);
    }
}

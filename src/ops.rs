use self::Op::*;
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    ADD,
    SUB,
    MUL,
    DIV,
    SDIV,
    MOD,
    SMOD,
    EXP,
    SIGNEXT,
    LT,
    GT,
    // SLT, pub(crate)
    // SGT, pub(crate)
    EQ,
    // AND,
    // OR,
    // XOR,
    BYTE,
    SHL,
    SHR,
    SAR,
    ADDMOD,
    MULMOD,
}

impl Op {

    pub fn iterator() -> Iter<'static, Op> {
        const OPS: [Op; 18] = [
            ADD,
            SUB,
            MUL,
            DIV,
            SDIV,
            MOD,
            SMOD,
            EXP,
            SIGNEXT,
            LT,
            GT,
            EQ,
            BYTE,
            SHL,
            SHR,
            SAR,
            ADDMOD,
            MULMOD,
        ];
        OPS.iter()
    }
}
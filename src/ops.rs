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

    // "high value" iter
    // this iter just returns the more complex operations
    // so we can focus the fuzzing where it's needed
    pub fn hv_iter() -> Iter<'static, Op> {
        const OPS: [Op; 9] = [
            // ADD,
            // SUB,
            // MUL,
            DIV,
            SDIV,
            MOD,
            SMOD,
            EXP,
            SIGNEXT,
            // LT,
            // GT,
            // EQ,
            // BYTE,
            // SHL,
            // SHR,
            SAR,
            ADDMOD,
            MULMOD,
        ];
        OPS.iter()
    }
}
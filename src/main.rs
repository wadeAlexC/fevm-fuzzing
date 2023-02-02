// use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::panic;
use std::ptr;

mod golang_ops;
use golang_ops::*;

// extern crate fil_actor_evm;
use fil_actor_evm::interpreter::instructions::arithmetic::*;
use fil_actor_evm::interpreter::instructions::boolean::*;
use fil_actor_evm::interpreter::instructions::bitwise::*;
use fil_actor_evm::interpreter::uints::U256;

use honggfuzz::fuzz;

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
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
}

#[derive(Debug)]
enum TernOp {
    ADDMOD,
    MULMOD,
}

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < 2 {
                return;
            }

            // Convert input data to 2 U256 values we can perform operations on
            let initial = to_u256(data);
            let fuzz_values = gen_alts(initial);
            let fuzz_values = fuzz_values.as_slice();

            // Capture any panics so we can control the error message we spit out
            match panic::catch_unwind(|| {
                try_binop(fuzz_values, BinOp::ADD).unwrap();
                try_binop(fuzz_values, BinOp::SUB).unwrap();
                try_binop(fuzz_values, BinOp::MUL).unwrap();
                try_binop(fuzz_values, BinOp::DIV).unwrap();
                try_binop(fuzz_values, BinOp::SDIV).unwrap();
                try_binop(fuzz_values, BinOp::MOD).unwrap();
                try_binop(fuzz_values, BinOp::SMOD).unwrap();
                try_binop(fuzz_values, BinOp::EXP).unwrap();
                try_binop(fuzz_values, BinOp::SIGNEXT).unwrap();
                try_binop(fuzz_values, BinOp::LT).unwrap();
                try_binop(fuzz_values, BinOp::GT).unwrap();
                try_binop(fuzz_values, BinOp::EQ).unwrap();
                try_binop(fuzz_values, BinOp::BYTE).unwrap();
                try_binop(fuzz_values, BinOp::SHL).unwrap();
                try_binop(fuzz_values, BinOp::SHR).unwrap();
                try_binop(fuzz_values, BinOp::SAR).unwrap();
            }) {
                Ok(()) => return,
                Err(err) => {
                    panic!(
                        "Got error for operation using initial values {} and {}. Err: {:?}",
                        initial[0], initial[1], err
                    );
                }
            }
        });
    }
}

// Generate arithmetic input from honggfuzz's random data
fn to_u256(data: &[u8]) -> [U256; 2] {
    // Split input in half
    let (in_a, in_b) = (&data[..data.len() / 2], &data[data.len() / 2..]);

    // Pad each input array to be len 32
    let mut arr_a = [0u8; 32];
    let mut len = in_a.len();
    if len > 32 {
        len = 32;
    }
    arr_a[..len].clone_from_slice(&in_a[..len]);

    let mut arr_b = [0u8; 32];
    let mut len = in_b.len();
    if len > 32 {
        len = 32;
    }
    arr_b[..len].clone_from_slice(&in_b[..len]);

    // Get a and b
    let a = U256::from_big_endian(&arr_a);
    let b = U256::from_big_endian(&arr_b);
    return [a, b];
}

// Performs arithmetic operation on 2 U256 values using both
// FEVM and Geth implementations, then compares the result
fn try_binop(values: &[[U256; 2]], op: BinOp) -> Result<(), String> {
    for value in values.iter().cloned() {
        let (fevm_result, go_result) = match op {
            BinOp::ADD => (add(value[0], value[1]), golang_op(value, op)?),
            BinOp::SUB => (sub(value[0], value[1]), golang_op(value, op)?),
            BinOp::MUL => (mul(value[0], value[1]), golang_op(value, op)?),
            BinOp::DIV => (div(value[0], value[1]), golang_op(value, op)?),
            BinOp::SDIV => (sdiv(value[0], value[1]), golang_op(value, op)?),
            BinOp::MOD => (modulo(value[0], value[1]), golang_op(value, op)?),
            BinOp::SMOD => (smod(value[0], value[1]), golang_op(value, op)?),
            BinOp::EXP => (exp(value[0], value[1]), golang_op(value, op)?),
            BinOp::SIGNEXT => (signextend(value[0], value[1]), golang_op(value, op)?),
            BinOp::LT => (lt(value[0], value[1]), golang_op(value, op)?),
            BinOp::GT => (gt(value[0], value[1]), golang_op(value, op)?),
            BinOp::EQ => (eq(value[0], value[1]), golang_op(value, op)?),
            BinOp::BYTE => (byte(value[0], value[1]), golang_op(value, op)?),
            BinOp::SHL => (shl(value[0], value[1]), golang_op(value, op)?),
            BinOp::SHR => (shr(value[0], value[1]), golang_op(value, op)?),
            BinOp::SAR => (sar(value[0], value[1]), golang_op(value, op)?),
            _ => return Err(format!("not implemented yet")),
        };

        if fevm_result != go_result {
            return Err(format!(
                "fevm and geth disagree for op {:?}. in0: {}, in1: {}, fevm res: {}, go res: {}",
                op, value[0], value[1], fevm_result, go_result
            ));
        }
    }

    Ok(())
}

// Takes the initial values we got from honggfuzz and
// adds a few additional options in case the input wasn't
// too interesting
fn gen_alts(initial: [U256; 2]) -> Vec<[U256; 2]> {
    let mut result = Vec::new();
    result.push(initial);

    // Swap values
    result.push([initial[1], initial[0]]);

    // Negate values
    result.push([!initial[0], !initial[1]]);

    // Replace a value with I128_MIN
    result.push([initial[0], U256::I128_MIN]);

    // Replace a value with U256::MAX
    result.push([initial[0], U256::max_value()]);

    // Replace a value with u64::MAX
    result.push([initial[0], U256::from_u64(u64::MAX)]);

    // Duplicate each operation and flip operands
    // let duplicates: Vec<[U256; 2]> = result.iter().map(|v| [v[1], v[0]]).collect();
    // result.extend(duplicates);
    // result.extend::<Vec<[U256; 2]>>(
    //     result
    //             .iter()
    //             .map(|v| [v[1], v[0]])
    //             .collect());

    return result;
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_setup() {
        let a = U256::from(2i64);
        let b = U256::from(4i64);
        let initial = [a, b];
        let fuzz_values = gen_alts(initial);
        let fuzz_values = fuzz_values.as_slice();

        println!("Got {} values:", fuzz_values.len());
        for v in fuzz_values {
            println!("{:x} | {:x}", v[0], v[1]);
        }

        match try_binop(fuzz_values, BinOp::ADD) {
            Ok(_) => println!("Impls agree on ADD"),
            Err(e) => panic!("Got error for ADD: {:?}", e),
        }

        println!("Running individual operations:");

        
        let fevm_result = add(a, b);
        let go_result = golang_op(initial, BinOp::ADD).unwrap();
        println!("ADD {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = sub(a, b);
        let go_result = golang_op(initial, BinOp::SUB).unwrap();
        println!("SUB {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = mul(a, b);
        let go_result = golang_op(initial, BinOp::MUL).unwrap();
        println!("MUL {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = div(a, b);
        let go_result = golang_op(initial, BinOp::DIV).unwrap();
        println!("DIV {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = sdiv(a, b);
        let go_result = golang_op(initial, BinOp::SDIV).unwrap();
        println!("SDIV {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = modulo(a, b);
        let go_result = golang_op(initial, BinOp::MOD).unwrap();
        println!("MOD {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = smod(a, b);
        let go_result = golang_op(initial, BinOp::SMOD).unwrap();
        println!("SMOD {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = exp(a, b);
        let go_result = golang_op(initial, BinOp::EXP).unwrap();
        println!("EXP {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = signextend(a, b);
        let go_result = golang_op(initial, BinOp::SIGNEXT).unwrap();
        println!("SIGNEXT {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result, "{} signext {} | rust: {} | go: {}", a, b, fevm_result, go_result);

        let fevm_result = lt(a, b);
        let go_result = golang_op(initial, BinOp::LT).unwrap();
        println!("LT {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = gt(a, b);
        let go_result = golang_op(initial, BinOp::GT).unwrap();
        println!("GT {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = eq(a, b);
        let go_result = golang_op(initial, BinOp::EQ).unwrap();
        println!("EQ {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = byte(a, b);
        let go_result = golang_op(initial, BinOp::BYTE).unwrap();
        println!("BYTE {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = shl(a, b);
        let go_result = golang_op(initial, BinOp::SHL).unwrap();
        println!("SHL {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = shr(a, b);
        let go_result = golang_op(initial, BinOp::SHR).unwrap();
        println!("SHR {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = sar(a, b);
        let go_result = golang_op(initial, BinOp::SAR).unwrap();
        println!("SAR {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);
    }
}

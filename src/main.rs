use std::panic;

mod golang_ops;
use golang_ops::*;

mod ops;
use ops::*;

use fil_actor_evm::interpreter::instructions::arithmetic::*;
use fil_actor_evm::interpreter::instructions::boolean::*;
use fil_actor_evm::interpreter::instructions::bitwise::*;
use fil_actor_evm::interpreter::uints::U256;

use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            // Reject small inputs
            if data.len() < 10 {
                return;
            }

            // Convert input data to 2 U256 values we can perform operations on
            let initial = to_u256(data);
            let fuzz_values = gen_alts(initial);
            let fuzz_values = fuzz_values.as_slice();

            // Capture any panics so we can control the error message we spit out
            match panic::catch_unwind(|| {
                // For each arithmetic operation, call both FEVM and Geth arithmetic
                // methods and compare the results against each other
                for op in Op::iterator() {
                    match try_op(fuzz_values, op) {
                        Ok(()) => continue,
                        Err(e) => return Err(format!("Got err with op {:?}. Err: {}", op, e)),
                    }
                }
                Ok(())
            }) {
                Ok(_) => return,
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
fn to_u256(data: &[u8]) -> [U256; 3] {
    // Split input in thirds
    let third = data.len() / 3;
    let (in_a, in_b, in_c) = (&data[..third], &data[third..third*2], &data[third*2..]);

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

    let mut arr_c = [0u8; 32];
    let mut len = in_c.len();
    if len > 32 {
        len = 32;
    }
    arr_c[..len].clone_from_slice(&in_c[..len]);

    // Get a, b, and c
    let a = U256::from_big_endian(&arr_a);
    let b = U256::from_big_endian(&arr_b);
    let c = U256::from_big_endian(&arr_c);
    return [a, b, c];
}

// Performs arithmetic operation on 2 U256 values using both
// FEVM and Geth implementations, then compares the result
fn try_op(values: &[[U256; 3]], op: &Op) -> Result<(), String> {
    for value in values.iter().cloned() {
        let (fevm_result, go_result) = match op {
            Op::ADD => (add(value[0], value[1]), golang_op(value, op)?),
            Op::SUB => (sub(value[0], value[1]), golang_op(value, op)?),
            Op::MUL => (mul(value[0], value[1]), golang_op(value, op)?),
            Op::DIV => (div(value[0], value[1]), golang_op(value, op)?),
            Op::SDIV => (sdiv(value[0], value[1]), golang_op(value, op)?),
            Op::MOD => (modulo(value[0], value[1]), golang_op(value, op)?),
            Op::SMOD => (smod(value[0], value[1]), golang_op(value, op)?),
            Op::EXP => (exp(value[0], value[1]), golang_op(value, op)?),
            Op::SIGNEXT => (signextend(value[0], value[1]), golang_op(value, op)?),
            Op::LT => (lt(value[0], value[1]), golang_op(value, op)?),
            Op::GT => (gt(value[0], value[1]), golang_op(value, op)?),
            Op::EQ => (eq(value[0], value[1]), golang_op(value, op)?),
            Op::BYTE => (byte(value[0], value[1]), golang_op(value, op)?),
            Op::SHL => (shl(value[0], value[1]), golang_op(value, op)?),
            Op::SHR => (shr(value[0], value[1]), golang_op(value, op)?),
            Op::SAR => (sar(value[0], value[1]), golang_op(value, op)?),
            Op::ADDMOD => (addmod(value[0], value[1], value[2]), golang_op(value, op)?),
            Op::MULMOD => (mulmod(value[0], value[1], value[2]), golang_op(value, op)?),
            // _ => return Err(format!("not implemented yet")),
        };

        if fevm_result != go_result {
            return Err(format!(
                "fevm and geth disagree for op {:?}. in0: {}, in1: {}, in2: {}, fevm res: {}, go res: {}",
                op, value[0], value[1], value[2], fevm_result, go_result
            ));
        }
    }

    Ok(())
}

// Takes the initial values we got from honggfuzz and
// adds a few additional options in case the input wasn't
// too interesting
fn gen_alts(initial: [U256; 3]) -> Vec<[U256; 3]> {
    let mut result = Vec::new();
    result.push(initial);

    // Swap values
    result.push([initial[2], initial[0], initial[1]]);

    // Negate values
    result.push([!initial[0], !initial[1], !initial[2]]);

    // Replace values with I128::MIN
    result.push([initial[0], U256::I128_MIN, initial[2]]);
    result.push([initial[1], initial[0], U256::I128_MIN]);

    // Replace values with U256::MAX
    result.push([initial[0], U256::max_value(), initial[2]]);
    result.push([initial[2], initial[1], U256::max_value()]);

    // Replace values with u64::MAX
    result.push([initial[0], U256::from_u64(u64::MAX), initial[2]]);
    result.push([initial[1], initial[2], U256::from_u64(u64::MAX)]);

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
        let c = U256::from(6i64);
        let initial = [a, b, c];
        let fuzz_values = gen_alts(initial);
        let fuzz_values = fuzz_values.as_slice();

        println!("Got {} values:", fuzz_values.len());
        for v in fuzz_values {
            println!("{:x} | {:x} | {:x}", v[0], v[1], v[2]);
        }

        for op in Op::iterator() {
            match try_op(fuzz_values, op) {
                Ok(()) => println!("Impls agree on op {:?}", op),
                Err(e) => panic!("Got err with op {:?}. Err: {}", op, e),
            }
        }

        println!("Running individual operations:");
        
        let fevm_result = add(a, b);
        let go_result = golang_op(initial, &Op::ADD).unwrap();
        println!("ADD {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = sub(a, b);
        let go_result = golang_op(initial, &Op::SUB).unwrap();
        println!("SUB {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = mul(a, b);
        let go_result = golang_op(initial, &Op::MUL).unwrap();
        println!("MUL {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = div(a, b);
        let go_result = golang_op(initial, &Op::DIV).unwrap();
        println!("DIV {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = sdiv(a, b);
        let go_result = golang_op(initial, &Op::SDIV).unwrap();
        println!("SDIV {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = modulo(a, b);
        let go_result = golang_op(initial, &Op::MOD).unwrap();
        println!("MOD {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = smod(a, b);
        let go_result = golang_op(initial, &Op::SMOD).unwrap();
        println!("SMOD {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = exp(a, b);
        let go_result = golang_op(initial, &Op::EXP).unwrap();
        println!("EXP {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = signextend(a, b);
        let go_result = golang_op(initial, &Op::SIGNEXT).unwrap();
        println!("SIGNEXT {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result, "{} signext {} | rust: {} | go: {}", a, b, fevm_result, go_result);

        let fevm_result = lt(a, b);
        let go_result = golang_op(initial, &Op::LT).unwrap();
        println!("LT {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = gt(a, b);
        let go_result = golang_op(initial, &Op::GT).unwrap();
        println!("GT {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = eq(a, b);
        let go_result = golang_op(initial, &Op::EQ).unwrap();
        println!("EQ {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = byte(a, b);
        let go_result = golang_op(initial, &Op::BYTE).unwrap();
        println!("BYTE {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = shl(a, b);
        let go_result = golang_op(initial, &Op::SHL).unwrap();
        println!("SHL {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = shr(a, b);
        let go_result = golang_op(initial, &Op::SHR).unwrap();
        println!("SHR {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = sar(a, b);
        let go_result = golang_op(initial, &Op::SAR).unwrap();
        println!("SAR {} {} == {}", a, b, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = addmod(a, b, c);
        let go_result = golang_op(initial, &Op::ADDMOD).unwrap();
        println!("ADDMOD {} {} {} == {}", a, b, c, fevm_result);
        assert_eq!(fevm_result, go_result);

        let fevm_result = mulmod(a, b, c);
        let go_result = golang_op(initial, &Op::MULMOD).unwrap();
        println!("MULMOD {} {} {} == {}", a, b, c, fevm_result);
        assert_eq!(fevm_result, go_result);
    }
}

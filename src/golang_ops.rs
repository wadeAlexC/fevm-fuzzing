use std::ptr;
use fil_actor_evm::interpreter::uints::U256;

use crate::Op;

#[link(name = "calculate")]
extern "C" {
    fn Add(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Sub(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Mul(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Div(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn SDiv(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Mod(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn SMod(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Exp(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn SignExt(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Lt(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Gt(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Eq(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Byte(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Shl(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Shr(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn Sar(a: *const u8, a_len: i32, b: *const u8, b_len: i32, out: *mut *mut u8, size: *mut i32) -> i32;
    fn AddMod(
        a: *const u8, a_len: i32, 
        b: *const u8, b_len: i32,
        c: *const u8, c_len: i32, 
        out: *mut *mut u8, size: *mut i32
    ) -> i32;
    fn MulMod(
        a: *const u8, a_len: i32, 
        b: *const u8, b_len: i32,
        c: *const u8, c_len: i32, 
        out: *mut *mut u8, size: *mut i32
    ) -> i32;
}

pub fn golang_op(values: [U256; 3], op: &Op) -> Result<U256, String> {
    let buf_a = values[0].to_bytes().to_vec();
    let buf_b = values[1].to_bytes().to_vec();
    let buf_c = values[2].to_bytes().to_vec();
    
    unsafe {
        let mut buf: *mut u8 = ptr::null_mut();
        let mut size: i32 = 0;
        let result = match match op {
            Op::ADD => Add(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::SUB => Sub(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::MUL => Mul(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::DIV => Div(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::SDIV => SDiv(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::MOD => Mod(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::SMOD => SMod(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::EXP => Exp(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::SIGNEXT => SignExt(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::LT => Lt(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::GT => Gt(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::EQ => Eq(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::BYTE => Byte(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::SHL => Shl(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::SHR => Shr(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::SAR => Sar(buf_a.as_ptr(), buf_a.len() as i32, buf_b.as_ptr(), buf_b.len() as i32, &mut buf, &mut size),
            Op::ADDMOD => AddMod(
                buf_a.as_ptr(), buf_a.len() as i32, 
                buf_b.as_ptr(), buf_b.len() as i32, 
                buf_c.as_ptr(), buf_c.len() as i32,
                &mut buf, &mut size
            ),
            Op::MULMOD => MulMod(
                buf_a.as_ptr(), buf_a.len() as i32, 
                buf_b.as_ptr(), buf_b.len() as i32, 
                buf_c.as_ptr(), buf_c.len() as i32,
                &mut buf, &mut size
            ),
            // _ => return Err(format!("go op {:?} not implemented yet", op)),
        } {
            0 => Vec::from_raw_parts(buf, size as usize, size as usize).clone(),
            e => return Err(format!("Go returned error value for op {:?}. Err: {}", op, e)),
        };

       let result = result.as_slice();
       return Ok(U256::from_big_endian(result));
    }
}
package main

// #include <stdint.h>
// typedef const uint8_t* buf_t;
import "C"

import (
	"unsafe"

	"github.com/holiman/uint256"
)

//export Add
func Add(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	// result := addInner(arrA, arrB)
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0).Add(a, b)
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Sub
func Sub(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	// result := subInner(arrA, arrB)
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0).Sub(a, b)
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Mul
func Mul(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0).Mul(a, b)
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Div
func Div(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0).Div(a, b)
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export SDiv
func SDiv(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0).SDiv(a, b)
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Mod
func Mod(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0).Mod(a, b)
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export SMod
func SMod(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0).SMod(a, b)
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Exp
func Exp(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0).Exp(a, b)
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export SignExt
func SignExt(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0).ExtendSign(b, a) // Geth flips these in instructions.go
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Lt
func Lt(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0)
	if a.Lt(b) {
		resInt = uint256.NewInt(1)
	}
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Gt
func Gt(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0)
	if a.Gt(b) {
		resInt = uint256.NewInt(1)
	}
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Eq
func Eq(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0)
	if a.Eq(b) {
		resInt = uint256.NewInt(1)
	}
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Byte
func Byte(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := b.Byte(a) // Opposite order than normal
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Shl
func Shl(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0)
	if a.LtUint64(256) {
		resInt = resInt.Lsh(b, uint(a.Uint64()))
	}
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Shr
func Shr(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0)
	if a.LtUint64(256) {
		resInt = resInt.Rsh(b, uint(a.Uint64()))
	}
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export Sar
func Sar(bufA C.buf_t, lenA C.int, bufB C.buf_t, lenB C.int, out **C.uint8_t, size *C.int) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	resInt := uint256.NewInt(0)
	if a.GtUint64(256) {
		if b.Sign() >= 0 {
			resInt = resInt.Clear()
		} else {
			resInt = resInt.SetAllOne()
		}
	} else {
		n := uint(a.Uint64())
		resInt = resInt.SRsh(b, n)
	}
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export AddMod
func AddMod(
	bufA C.buf_t, lenA C.int,
	bufB C.buf_t, lenB C.int,
	bufC C.buf_t, lenC C.int,
	out **C.uint8_t, size *C.int,
) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)
	arrC := toBuf(bufC, lenC)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	c := uint256.NewInt(0).SetBytes(arrC)
	resInt := uint256.NewInt(0)
	if c.IsZero() {
		resInt = resInt.Clear()
	} else {
		resInt = resInt.AddMod(a, b, c)
	}
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

//export MulMod
func MulMod(
	bufA C.buf_t, lenA C.int,
	bufB C.buf_t, lenB C.int,
	bufC C.buf_t, lenC C.int,
	out **C.uint8_t, size *C.int,
) (res C.int) {
	defer func() {
		if rerr := recover(); rerr != nil {
			res = -1
		}
	}()

	arrA := toBuf(bufA, lenA)
	arrB := toBuf(bufB, lenB)
	arrC := toBuf(bufC, lenC)

	// Calculate result
	a := uint256.NewInt(0).SetBytes(arrA)
	b := uint256.NewInt(0).SetBytes(arrB)
	c := uint256.NewInt(0).SetBytes(arrC)
	resInt := uint256.NewInt(0).MulMod(a, b, c)
	result := resInt.Bytes32()

	// Write result to pointer
	*out = (C.buf_t)(C.CBytes(result[:]))
	*size = C.int(len(result))

	return 0
}

func toBuf(buf C.buf_t, length C.int) []byte {
	return C.GoBytes(unsafe.Pointer(buf), C.int(length))
}

func main() {}

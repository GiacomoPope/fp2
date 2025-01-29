"""
This file, given a prime p = 3 mod 4 computes all the following constants needed
for the macros fp_gen.rs and fp2_gen.rs

//      N (usize)                 the number of words needed
//      BITLEN (usize)            modulus length in bits
//      MODULUS ([u64; N])        modulus p (little-endian)
//      HALF_MODULUS ([u64; N])   (p + 1)/2 (little-endian)
//      R_VAL ([u64; N])          2^(64*N) mod p
//      DR_VAL ([u64; N])         2*R_VAL mod p = 2^(64*N+1) mod p
//      TR_VAL ([u64; N])         3*R_VAL mod p = 3*2^(64*N) mod p
//      QR_VAL ([u64; N])         4*R_VAL mod p = 2^(64*N+2) mod p
//      R2_VAL ([u64; N])         R_VAL^2 mod p = 2^(2*64*N) mod p
//      P0I (u64)                 -1/p mod 2^64
//      TFIXDIV_VAL               corrective factor for division
//      TDEC_VAL                  2^(64*(2*N-1)) mod p
//      SQRT_EH, SQRT_EL          encoding of (p + 1)/4
//      FOURTH_ROOT_EH, FOURTH_ROOT_EL          encoding of (p + 1)/8
//      P1 (u64)                  floor(p / 2^(BITLEN - 32))
//      P1DIV_M (u64)             1 + floor((2^32 - P1)*2^64 / P1)
//      NQR_RE_VAL                NQR_RE + i is a non-square in GF(p^2)
"""

from sage.all import GF, floor, proof, ceil, inverse_mod

proof.all(False)
ea = 74
eb = 41
A = 2**ea
B = 3**eb
p = A * B - 1
p = 5 * 2**248 - 1
assert p.is_prime()
words = ceil(p.nbits() / 64)


def to_little_u64(n):
    res = []
    while n:
        tmp = n % 2**64
        res.append(hex(tmp))
        n >>= 64
    l = fmt_little_u64(res)
    return fmt_list(l)


def fmt_little_u64(res):
    out = []
    for r in res:
        num = r[2:]
        new_num = num.zfill(16)
        new_num = new_num.upper()
        new_num = "0x" + new_num
        out.append(new_num)
    while len(out) != words:
        out.append("0x" + "0" * 16)
    return out


def fmt_list(l):
    l = str(l)
    l = l.replace("[", "")
    l = l.replace("]", "")
    l = l.replace("'", "")
    return l


BITLEN = p.nbits()
N = words
MODULUS = to_little_u64(p)
P_PLUS_ONE = to_little_u64(p + 1)
HALF_MODULUS = to_little_u64((p + 1) // 2)
R_VAL = to_little_u64(2 ** (64 * N) % p)
MINUS_R_VAL = to_little_u64(-(2 ** (64 * N)) % p)
DR_VAL = to_little_u64(2 ** (64 * N + 1) % p)
TR_VAL = to_little_u64(3 * 2 ** (64 * N) % p)
QR_VAL = to_little_u64(2 ** (64 * N + 2) % p)
R2_VAL = to_little_u64(2 ** (2 * 64 * N) % p)
P0I = inverse_mod(-p, 2**64)

"""
let n1 = floor((2*BITLEN - 34) / 31)
let n2 = 2*BITLEN - 31*n1 - 2
TFIXDIV_VAL = 2^(33*n1 + 64 - n2 + 2*64*N) mod p
"""
n1 = floor((2 * BITLEN - 34) // 31)
n2 = 2 * BITLEN - 31 * n1 - 2
TFIXDIV_VAL = 2 ** (33 * n1 + 64 - n2 + 2 * 64 * N) % p
TFIXDIV_VAL = to_little_u64(TFIXDIV_VAL)

TDEC_VAL = to_little_u64(2 ** (64 * (2 * N - 1)) % p)

WIN_LEN = 5
e = (p + 1) // 4
SQRT_EL = BITLEN
while True:
    mod = 2 ** (5 * SQRT_EL)
    if e % mod == 0:
        break
    SQRT_EL -= 1

SQRT_EH = e // 2 ^ (5 * SQRT_EL)
SQRT_EH = SQRT_EH.digits(2**5)

e = (p + 1) // 8
FOURTH_ROOT_EL = BITLEN
while True:
    mod = 2 ** (5 * FOURTH_ROOT_EL)
    if e % mod == 0:
        break
    FOURTH_ROOT_EL -= 1

FOURTH_ROOT_EH = e // 2 ^ (5 * FOURTH_ROOT_EL)
FOURTH_ROOT_EH = FOURTH_ROOT_EH.digits(2**5)

P1 = floor(p // 2 ** (BITLEN - 32))
P1DIV_M = 1 + ((2**32 - P1) * 2**64 // P1)


F = GF(p**2, name="i", modulus=[1, 0, 1])
NQR_RE_VAL = 0
while True:
    NQR_RE_VAL += 1
    if not F([NQR_RE_VAL, 1]).is_square():
        break

assert not F([NQR_RE_VAL, 1]).is_square()
NQR_RE_VAL = to_little_u64(2 ** (64 * N) * NQR_RE_VAL % p)

str = f"""
// Fp{BITLEN}: a finite field element GF(p) with p = 3 mod 4. 
// Contents are opaque, all functions are constant-time.
// Macro input generated with scripts/gen_fp.sage
crate::finitefield::fp_gen::define_fp_core!(
    type_name = Fp{BITLEN},
    words = {N}_usize,
    bit_len = {BITLEN}_usize,
    modulus = [{MODULUS}],
    half_modulus = [{HALF_MODULUS}],
    mont_r = [{R_VAL}],
    neg_r = [{MINUS_R_VAL}],
    two_r = [{DR_VAL}],
    three_r = [{TR_VAL}],
    four_r = [{QR_VAL}],
    r_sqr = [{R2_VAL}],
    minus_p_inv = {P0I}_u64,
    div_correction = [{TFIXDIV_VAL}],
    reduce_const = [{TDEC_VAL}],
    window_len = {WIN_LEN}_usize,
    sqrt_el = {SQRT_EL}_usize,
    sqrt_eh = {SQRT_EH},
    fourth_root_el = {FOURTH_ROOT_EL}_usize,
    fourth_root_eh = {FOURTH_ROOT_EH},
    p1 = {P1}_u64,
    p1_div_m = {P1DIV_M}_u64,
);

// Fp{BITLEN}Ext: a finite field element GF(p^2) with modulus x^2 + 1. 
// Contents are opaque, all functions are constant-time.
// Macro input generated with scripts/gen_fp.sage
crate::finitefield::fp2_gen::define_fp2_core!(
    type_name = Fp{BITLEN}Ext,
    base_field = crate::fields::Fp{BITLEN},
    nqr_re = [{NQR_RE_VAL}]
);

#[cfg(test)]
mod fp{BITLEN}_tests {{
    use crate::fields::Fp{BITLEN};
    use crate::fields::Fp{BITLEN}Ext;

    crate::finitefield::fp_gen::define_fp_tests!(Fp{BITLEN});
    crate::finitefield::fp2_gen::define_fp2_tests!(Fp{BITLEN}, Fp{BITLEN}Ext);
}}
"""

print(str)

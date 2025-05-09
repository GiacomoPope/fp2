"""
A small SageMath script to generate the macro parameters for a given prime
"""

from sage.all import GF, proof, ceil

proof.all(False)

def fmt_little_u64(res, words):
    out = []
    for r in res:
        num = r[2:]
        new_num = num.zfill(16)
        new_num = new_num.upper()
        new_num = "0x" + new_num + "u64"
        out.append(new_num)
    while len(out) != words:
        out.append("0x" + "0" * 16 + "u64")
    return out

def fmt_list(l):
    l = str(l)
    l = l.replace("[", "")
    l = l.replace("]", "")
    l = l.replace("'", "")
    return l

def to_little_u64(n, words):

    res = []
    while n:
        tmp = n % 2**64
        res.append(hex(tmp))
        n >>= 64
    l = fmt_little_u64(res, words)
    return fmt_list(l)


if __name__ == "__main__":
    p = 65 * 2**376 - 1
    assert p.is_prime()
    assert p % 4 == 3
    words = ceil(p.nbits() / 64)
    BITLEN = p.nbits()
    MODULUS = to_little_u64(p, words)

    Fp2 = GF(p**2, names="i", modulus=[1,0,1])
    nqr = 1
    for i in range(1, 1000):
        x = Fp2([i, 1])
        if not x.is_square():
            nqr = i
            break

    str = f"""
    // Fp{BITLEN}: a finite field element GF(p) with p = 3 mod 4.
    // Contents are opaque, all functions are constant-time.
    fp2_rs::define_fp_core!(
        typename = Fp{BITLEN},
        modulus = [{MODULUS}],
    );

    // Fp{BITLEN}Ext: a finite field element GF(p^2) with modulus x^2 + 1.
    // Contents are opaque, all functions are constant-time.
    fp2_rs::define_fp2_core!(
        typename = Fp{BITLEN}Ext,
        base_field = Fp{BITLEN},
    );

    fp2_rs::define_fp_tests!(Fp139);
    fp2_rs::define_fp2_tests!(Fp{BITLEN}Ext, {nqr});
    """

    print(str)

//! Scalar functions in \Z/(2^252 + 27742317777372353535851937790883648493)

#[cfg(any(any(target_arch = "arm"), feature = "force-32bits"))]
mod scalar32;

#[cfg(not(any(any(target_arch = "arm"), feature = "force-32bits")))]
mod scalar64;

#[cfg(any(any(target_arch = "arm"), feature = "force-32bits"))]
pub use scalar32::*;

#[cfg(not(any(any(target_arch = "arm"), feature = "force-32bits")))]
pub use scalar64::*;

impl Scalar {
    #[allow(clippy::needless_range_loop)]
    pub(crate) fn slide(&self) -> [i8; 256] {
        let mut r = self.bits();
        for i in 0..256 {
            if r[i] != 0 {
                for b in 1..core::cmp::min(7, 256 - i) {
                    if r[i + b] != 0 {
                        if r[i] + (r[i + b] << b) <= 15 {
                            r[i] += r[i + b] << b;
                            r[i + b] = 0;
                        } else if r[i] - (r[i + b] << b) >= -15 {
                            r[i] -= r[i + b] << b;
                            for k in i + b..256 {
                                if r[k] == 0 {
                                    r[k] = 1;
                                    break;
                                }
                                r[k] = 0;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MulAddIv {
        a: [u8; 32],
        b: [u8; 32],
        c: [u8; 32],
        r: [u8; 32],
    }

    #[test]
    fn muladd_ivs() {
        let ivs = [
            // A * 0 + C
            MulAddIv {
                a: [
                    1, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0,
                ],
                b: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
                c: [
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0,
                ],
                r: [
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0,
                ],
            },
            // B * 1 + 0
            MulAddIv {
                a: [
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
                b: [
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15, 26, 17, 18, 19, 20, 1, 2, 3, 4, 5,
                    6, 7, 1, 2, 3, 4, 5, 6, 1,
                ],
                c: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
                r: [
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15, 26, 17, 18, 19, 20, 1, 2, 3, 4, 5,
                    6, 7, 1, 2, 3, 4, 5, 6, 1,
                ],
            },
            // B * 1 + C
            MulAddIv {
                a: [
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ],
                b: [
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5,
                    6, 7, 1, 2, 3, 4, 5, 6, 1,
                ],
                c: [
                    10, 20, 30, 40, 50, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0,
                ],
                r: [
                    11, 22, 33, 44, 55, 66, 7, 8, 9, 10, 12, 13, 15, 16, 17, 18, 19, 20, 1, 2, 3,
                    4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 1,
                ],
            },
            MulAddIv {
                a: [
                    0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 124, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0,
                ],
                b: [
                    0, 0, 0, 0, 1, 2, 0, 4, 0, 0, 0, 8, 16, 32, 234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 1,
                ],
                c: [
                    0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 1, 1, 1,
                ],
                r: [
                    199, 123, 129, 187, 98, 220, 200, 84, 69, 52, 169, 69, 239, 116, 129, 54, 19,
                    231, 107, 167, 4, 33, 71, 87, 236, 240, 0, 113, 70, 120, 168, 0,
                ],
            },
        ];

        for (i, iv) in ivs.iter().enumerate() {
            let out = muladd(
                &Scalar::from_bytes(&iv.a),
                &Scalar::from_bytes(&iv.b),
                &Scalar::from_bytes(&iv.c),
            );
            assert_eq!(iv.r, out.to_bytes(), "IV test {} failed", i);
        }
    }
}

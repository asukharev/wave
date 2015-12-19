pub trait Bits {
    fn to_binary(&self) -> u32;
}

impl Bits for f32 {
    fn to_binary(&self) -> u32 {
        if *self == 0.0f32 {
            0
        }
        else {
            let integer: u32 = self.abs().floor() as u32;
            let fraction = self.abs() - integer as f32;
            let mut e: f32 = 0.0f32;
            let sight: u32 = (if *self < 0f32 { 1 } else { 0 }) << 31;

            let mut pos: i8 = 0;
            let mut ib = 0u32;
            {
                if integer >= 1 {
                    let mut f = self.abs() as u32;
                    if (self.abs() as u32) >= 1 {
                        while f > 0 {
                            let b = f % 2;
                            ib |= b << (31 - (31 - pos));
                            f /= 2u32;
                            pos += 1;
                        }
                        ib = ib << (31 - { if pos == 0 { 0 } else { pos - 1 } });
                        ib = ib >> 8;
                        e = pos as f32 - 1.0f32;
                    }
                }
            }

            let mut fb = 0u32;
            {
                if fraction > 0.0f32 {
                    let mut f = fraction;
                    let mut sshift = 0;
                    while f > 0f32 && pos < 23 {
                        f *= 2.0f32;
                        if integer < 1 {
                            if fb == 0 && f as u32 == 0 {
                                // skip shift
                                sshift += 1;
                                f -= f.floor();
                            }
                            else {
                                fb |= (f as u32) << (31 - 8 - pos);
                                f -= f.floor();
                                pos += 1;
                            }
                        }
                        else {
                            fb |= (f as u32) << (31 - 8 - pos);
                            f -= f.floor();
                            pos += 1;
                        }
                    }
                    if sshift > 0 {
                        if (f * 2.0f32) as u32 == 0 { fb &= !1 } else { fb |= 1 };
                    }
                    if integer < 1 { e = self.abs().log2().floor(); };
                }
            }

            //
            let mut w: u32 = sight | ib | fb;
            for i in 1..9 { // clear bits
                w &= !(1 << (31 - i));
            }
            w |= ( ( 127f32 + e ) as u32 ) << 23; // e
            w
        }
    }
}

#[test]
fn float_to_binary() {
    println!("");

    let nums: Vec<(f32, u32)> = vec![
        ( 12.375, 0x41460000),
        (12.3751, 0x41460068 /*0x41460069*/),
        (    1.0, 0x3F800000),
        (   0.25, 0x3E800000),
        (    0.5, 0x3F000000),
        (    0.3, 0x3E99999A),
        (    0.2, 0x3E4CCCCD),
        (  0.003, 0x3B449BA6),
        // (    0.0, 0x00000000),
        (   -0.4, 0xBECCCCCD),
        (   -0.9, 0xBF666666),
        (   -1.0, 0xBF800000),
    ];

    for (num, check) in nums {
        let w = num.to_binary();
        assert_eq!(w, check);
    }
}

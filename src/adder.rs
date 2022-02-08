// fn adder(a: u32, b: u32) -> u32;

pub fn inc(i: u32) -> u32 {
    if (i & 1) == 0 {
        return i | 1;
    }
    return inc(i >> 1) << 1;
}

pub fn adder(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    adder(inc(a), b - 1)
}

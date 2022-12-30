pub struct SmallXXHash {
    primeA : u32,
    primeB : u32,
    primeC : u32,
    primeD : u32,
    primeE : u32,

    accumulator : u32
}

impl SmallXXHash {
    pub fn new(seed: i32) -> SmallXXHash {
        SmallXXHash {
            accumulator: seed as u32 + primeE
        }
    }

    pub fn eat(data : i32) {
        accumulator = RotateLeft(accumulator + data as u32 * primeC, 17) * primeD;
    }

    pub fn eat(data : u8) {
        accumulator = RotateLeft(accumulator + data as u8 * primeE, 11) * primeA;
    }

    pub fn rotateLeft(data: u32, steps: i32) -> u32 {
        (data << steps) | (data >> (32 - steps))
    }
}

impl Default for SmallXXHash {
    fn default() -> SmallXXHash {
        SmallXXHash {
            primeA: 0b10011110001101110111100110110001,
            primeB: 0b10000101111010111100101001110111,
            primeC: 0b11000010101100101010111000111101,
            primeD: 0b00100111110101001110101100101111,
            primeE: 0b00010110010101100110011110110001,
        }
    }
}
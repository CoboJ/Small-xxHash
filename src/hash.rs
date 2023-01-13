const _PRIME_A : u32 = 0b10011110001101110111100110110001;
const PRIME_B : u32 = 0b10000101111010111100101001110111;
const PRIME_C : u32 = 0b11000010101100101010111000111101;
const PRIME_D : u32 = 0b00100111110101001110101100101111;
const PRIME_E : u32 = 0b00010110010101100110011110110001;

pub struct SmallXXHash {
    accumulator : u32
}

impl SmallXXHash {
    pub fn new(seed: i32) -> SmallXXHash {
        SmallXXHash {
            accumulator: (seed as u32).wrapping_add(PRIME_E)
        }
    }

    pub fn eat(&mut self, data : i32) -> &mut Self {
        self.accumulator = self.accumulator.
                            wrapping_add((data as u32).wrapping_mul(PRIME_C)).
                            rotate_left(17).
                            wrapping_mul(PRIME_D);

        self
    }

    pub fn get(&self) -> u32 {
        let mut avalanche: u32 = self.accumulator;
		avalanche ^= avalanche.wrapping_shr(15);
		avalanche = avalanche.wrapping_mul(PRIME_B);
		avalanche ^= avalanche.wrapping_shr(13);
		avalanche = avalanche.wrapping_mul(PRIME_C);
		avalanche ^= avalanche.wrapping_shr(16);
		return avalanche;
    }
}
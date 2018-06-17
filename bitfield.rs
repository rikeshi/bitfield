use std::mem;

trait Bits<T> {
    fn new() -> Self;
    fn with_pattern(pattern: T) -> Self;
    fn set_all(&mut self);
    fn set_pattern(&mut self, pattern: T);
    fn set_bit(&mut self, index: usize);
    fn unset_all(&mut self);
    fn unset_bit(&mut self, index: usize);
    fn check_pattern(&self, pattern: T) -> bool;
    fn check_bit(&self, index: usize) -> bool;
}

struct BitField<T> {
    bits: T,
}

macro_rules! impl_Bits_BitField {
    (for $($t:ty),+) => {
        $(impl Bits<$t> for BitField<$t> {
            fn new() -> Self {
                BitField { bits: 0 }
            }
            fn with_pattern(pattern: $t) -> Self {
                BitField { bits: pattern }
            }
            fn set_all(&mut self) {
                self.bits = <$t>::max_value();
            }
            fn set_pattern(&mut self, pattern: $t) {
                self.bits = pattern;
            }
            fn set_bit(&mut self, index: usize) {
                if index < mem::size_of::<$t>() * 8 {
                    self.bits |= 1 << index;
                }
            }
            fn unset_all(&mut self) {
                self.bits = <$t>::min_value();
            }
            fn unset_bit(&mut self, index: usize) {
                if index < mem::size_of::<$t>() * 8 {
                    self.bits &= !(1 << index);
                }
            }
            fn check_pattern(&self, pattern: $t) -> bool {
                self.bits == pattern
            }
            fn check_bit(&self, index: usize) -> bool {
                if index < mem::size_of::<$t>() * 8 {
                    return self.bits >> index & 1 == 1
                }
                false
            }
        })*
    }
}

impl_Bits_BitField!(for usize, u8, u16, u32, u64);

impl Bits<Vec<bool>> for BitField<Vec<bool>> {
    fn new() -> Self {
        BitField { bits: Vec::new() }
    }
    fn with_pattern(pattern: Vec<bool>) -> Self {
        BitField { bits: pattern }
    }
    fn set_all(&mut self) {
        self.bits = vec![true; self.bits.len()];
    }
    fn set_pattern(&mut self, pattern: Vec<bool>) {
        self.bits = pattern;
    }
    fn set_bit(&mut self, index: usize) {
        if index >= self.bits.len() {
            let diff = index - self.bits.len();
            self.bits.reserve(diff + 1);
            for _i in 0..diff {
                self.bits.push(false);
            }
            self.bits.push(true)
        } else {
            self.bits[index] = true;
        }
    }
    fn unset_all(&mut self) {
        self.bits = vec![false; self.bits.len()];
    }
    fn unset_bit(&mut self, index: usize) {
        if index < self.bits.len() {
            self.bits[index] = false;
        }
    }
    fn check_pattern(&self, pattern: Vec<bool>) -> bool {
        self.bits == pattern
    }
    fn check_bit(&self, index: usize) -> bool {
        if index < self.bits.len() {
            return self.bits[index]
        }
        false
    }
}

impl Bits<Vec<u8>> for BitField<Vec<u8>> {
    fn new() -> Self {
        BitField { bits: Vec::new() }
    }
    fn with_pattern(pattern: Vec<u8>) -> Self {
        BitField { bits: pattern }
    }
    fn set_all(&mut self) {
        self.bits = vec![255; self.bits.len()];
    }
    fn set_pattern(&mut self, pattern: Vec<u8>) {
        self.bits = pattern;
    }
    fn set_bit(&mut self, index: usize) {
        if index / 8 >= self.bits.len() {
            let diff = index / 8 - self.bits.len();
            self.bits.reserve(diff + 1);
            for _i in 0..diff {
                self.bits.push(0);
            }
            self.bits.push(1 << index % 8)
        } else {
            self.bits[index / 8] |= 1 << index % 8;
        }
    }
    fn unset_all(&mut self) {
        self.bits = vec![0; self.bits.len()];
    }
    fn unset_bit(&mut self, index: usize) {
        if index / 8 < self.bits.len() {
            self.bits[index / 8] &= !(1 << index % 8);
        }
    }
    fn check_pattern(&self, pattern: Vec<u8>) -> bool {
        self.bits == pattern
    }
    fn check_bit(&self, index: usize) -> bool {
        if index / 8 < self.bits.len() {
            return self.bits[index / 8] >> index % 8 & 1 == 1
        }
        false
    }
}

fn test_bitfield_usize(bf: &mut BitField<usize>) {
    assert_eq!(0, bf.bits);
    bf.set_all();
    assert_eq!(<usize>::max_value(), bf.bits);
    bf.unset_all();
    assert_eq!(0, bf.bits);
    bf.set_pattern(0b1101);
    assert_eq!(0b1101, bf.bits);
    bf.set_bit(1);
    assert_eq!(0b1111, bf.bits);
    bf.unset_bit(<usize>::max_value());
    assert_eq!(0b1111, bf.bits);
    bf.unset_bit(2);
    assert_eq!(0b1011, bf.bits);
    bf.set_bit(<usize>::max_value());
    assert_eq!(0b1011, bf.bits);
    assert_eq!(true, bf.check_bit(0));
    assert_eq!(true, bf.check_bit(1));
    assert_eq!(false, bf.check_bit(2));
    assert_eq!(true, bf.check_bit(3));
    assert!(bf.check_pattern(0b1011));
    println!("All tests passed for BitField<usize>.");
}

fn test_bitfield_u8(bf: &mut BitField<u8>) {
    assert_eq!(0, bf.bits);
    bf.set_all();
    assert_eq!(<u8>::max_value(), bf.bits);
    bf.unset_all();
    assert_eq!(0, bf.bits);
    bf.set_pattern(0b1101);
    assert_eq!(0b1101, bf.bits);
    bf.set_bit(1);
    assert_eq!(0b1111, bf.bits);
    bf.unset_bit(<usize>::max_value());
    assert_eq!(0b1111, bf.bits);
    bf.unset_bit(2);
    assert_eq!(0b1011, bf.bits);
    bf.set_bit(<usize>::max_value());
    assert_eq!(0b1011, bf.bits);
    assert_eq!(true, bf.check_bit(0));
    assert_eq!(true, bf.check_bit(1));
    assert_eq!(false, bf.check_bit(2));
    assert_eq!(true, bf.check_bit(3));
    assert!(bf.check_pattern(0b1011));
    println!("All tests passed for BitField<u8>.");
}

fn test_bitfield_u16(bf: &mut BitField<u16>) {
    assert_eq!(0, bf.bits);
    bf.set_all();
    assert_eq!(<u16>::max_value(), bf.bits);
    bf.unset_all();
    assert_eq!(0, bf.bits);
    bf.set_pattern(0b1101);
    assert_eq!(0b1101, bf.bits);
    bf.set_bit(1);
    assert_eq!(0b1111, bf.bits);
    bf.unset_bit(<usize>::max_value());
    assert_eq!(0b1111, bf.bits);
    bf.unset_bit(2);
    assert_eq!(0b1011, bf.bits);
    bf.set_bit(<usize>::max_value());
    assert_eq!(0b1011, bf.bits);
    assert_eq!(true, bf.check_bit(0));
    assert_eq!(true, bf.check_bit(1));
    assert_eq!(false, bf.check_bit(2));
    assert_eq!(true, bf.check_bit(3));
    assert!(bf.check_pattern(0b1011));
    println!("All tests passed for BitField<u16>.");
}

fn test_bitfield_u32(bf: &mut BitField<u32>) {
    assert_eq!(0, bf.bits);
    bf.set_all();
    assert_eq!(<u32>::max_value(), bf.bits);
    bf.unset_all();
    assert_eq!(0, bf.bits);
    bf.set_pattern(0b1101);
    assert_eq!(0b1101, bf.bits);
    bf.set_bit(1);
    assert_eq!(0b1111, bf.bits);
    bf.unset_bit(<usize>::max_value());
    assert_eq!(0b1111, bf.bits);
    bf.unset_bit(2);
    assert_eq!(0b1011, bf.bits);
    bf.set_bit(<usize>::max_value());
    assert_eq!(0b1011, bf.bits);
    assert_eq!(true, bf.check_bit(0));
    assert_eq!(true, bf.check_bit(1));
    assert_eq!(false, bf.check_bit(2));
    assert_eq!(true, bf.check_bit(3));
    assert!(bf.check_pattern(0b1011));
    println!("All tests passed for BitField<u32>.");
}

fn test_bitfield_u64(bf: &mut BitField<u64>) {
    assert_eq!(0, bf.bits);
    bf.set_all();
    assert_eq!(<u64>::max_value(), bf.bits);
    bf.unset_all();
    assert_eq!(0, bf.bits);
    bf.set_pattern(0b1101);
    assert_eq!(0b1101, bf.bits);
    bf.set_bit(1);
    assert_eq!(0b1111, bf.bits);
    bf.unset_bit(<usize>::max_value());
    assert_eq!(0b1111, bf.bits);
    bf.unset_bit(2);
    assert_eq!(0b1011, bf.bits);
    bf.set_bit(<usize>::max_value());
    assert_eq!(0b1011, bf.bits);
    assert_eq!(true, bf.check_bit(0));
    assert_eq!(true, bf.check_bit(1));
    assert_eq!(false, bf.check_bit(2));
    assert_eq!(true, bf.check_bit(3));
    assert!(bf.check_pattern(0b1011));
    println!("All tests passed for BitField<u64>.");
}

fn test_bitfield_vec_bool(bf: &mut BitField<Vec<bool>>) {
    let v: Vec<bool> = Vec::new();
    assert_eq!(v, bf.bits);
    bf.set_pattern(vec![false, true, false]);
    assert_eq!(vec![false, true, false], bf.bits);
    bf.set_all();
    assert_eq!(vec![true; 3], bf.bits);
    bf.unset_all();
    assert_eq!(vec![false; 3], bf.bits);
    bf.set_bit(1);
    bf.set_bit(2);
    assert_eq!(vec![false, true, true], bf.bits);
    bf.unset_bit(4);
    assert_eq!(vec![false, true, true], bf.bits);
    bf.unset_bit(1);
    assert_eq!(vec![false, false, true], bf.bits);
    bf.set_bit(4);
    assert_eq!(vec![false, false, true, false, true], bf.bits);
    assert_eq!(false, bf.check_bit(0));
    assert_eq!(false, bf.check_bit(1));
    assert_eq!(true, bf.check_bit(2));
    assert_eq!(false, bf.check_bit(3));
    assert_eq!(true, bf.check_bit(4));
    assert!(bf.check_pattern(vec![false, false, true, false, true]));
    println!("All tests passed for BitField<Vec<bool>>.");
}

fn test_bitfield_vec_u8(bf: &mut BitField<Vec<u8>>) {
    let v: Vec<u8> = Vec::new();
    assert_eq!(v, bf.bits);
    bf.set_pattern(vec![0b1101]);
    assert_eq!(vec![0b1101], bf.bits);
    bf.set_all();
    assert_eq!(vec![255], bf.bits);
    bf.unset_all();
    assert_eq!(vec![0], bf.bits);
    bf.set_pattern(vec![0b1101]);
    bf.set_bit(1);
    assert_eq!(vec![0b1111], bf.bits);
    bf.unset_bit(16);
    assert_eq!(vec![0b1111], bf.bits);
    bf.unset_bit(2);
    assert_eq!(vec![0b1011], bf.bits);
    assert_eq!(true, bf.check_bit(0));
    assert_eq!(true, bf.check_bit(1));
    assert_eq!(false, bf.check_bit(2));
    assert_eq!(true, bf.check_bit(3));
    assert!(bf.check_pattern(vec![0b1011]));
    bf.set_bit(16);
    assert_eq!(vec![0b00001011, 0, 1], bf.bits);
    println!("All tests passed for BitField<Vec<u8>>.");
}

fn main() {
    let mut bf_usize: BitField<usize> = BitField::new();
    let mut bf_u8: BitField<u8> = BitField::new();
    let mut bf_u16: BitField<u16> = BitField::new();
    let mut bf_u32: BitField<u32> = BitField::new();
    let mut bf_u64: BitField<u64> = BitField::new();
    let mut bf_vec_bool: BitField<Vec<bool>> = BitField::new();
    let mut bf_vec_u8: BitField<Vec<u8>> = BitField::new();

    test_bitfield_usize(&mut bf_usize);
    test_bitfield_u8(&mut bf_u8);
    test_bitfield_u16(&mut bf_u16);
    test_bitfield_u32(&mut bf_u32);
    test_bitfield_u64(&mut bf_u64);
    test_bitfield_vec_bool(&mut bf_vec_bool);
    test_bitfield_vec_u8(&mut bf_vec_u8);
}

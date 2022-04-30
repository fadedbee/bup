/*
 * The 32 bytes are stored as a BigUint, for ease of conversion to different bases.
 *
 * Representations:
 * 
 * 32x base-256 symbols (Uint8Array) (256 bits)
 * 43x base-62 symbols (256.03 bits)
 * 22x base-62 symbols (130.99 bits) and 25x base-33 symbols (126.10 bits)
 *   (The base-33 part is capped at 62^21 so that the base-62 part resembles the pure base-62 
 *    representation.)
 * 
 */

use std::{collections::HashMap, hash::Hash, str};

use num::{BigUint, ToPrimitive};
use num::pow::pow;
use lazy_static::lazy_static;

// base33 and base62
const BASE16_ALPHA: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];
const BASE33_ALPHA: [char; 33] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F',
                                  'G','H','J','K','L','M','N','P','Q','R','S','T','V','W','X','Y',
                                  'Z'];
const BASE62_ALPHA: [char; 62] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F',
                                  'G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V',
                                  'W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l',
                                  'm','n','o','p','q','r','s','t','u','v','w','x','y','z'];
fn rev(alpha: &[char]) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for i in 0..alpha.len() {
        map.insert(alpha[i].clone(), i);
    }
    map
}

lazy_static! {
    static ref BASE16_REV: HashMap<char, usize> = rev(&BASE16_ALPHA); 
    static ref BASE33_REV: HashMap<char, usize> = rev(&BASE33_ALPHA); 
    static ref BASE62_REV: HashMap<char, usize> = rev(&BASE62_ALPHA); 
}

#[derive(Copy, Clone, Debug)]
enum Base {
    BASE16 = 16,
    BASE33 = 33,
    BASE62 = 62
}

fn big_to_base(mut big: BigUint, base: &Base, digits: usize) -> String {
    let mut out = String::new();
    let divider = &BigUint::from(*base as u64);
    while out.len() < digits {
        let index = (big.clone() % divider).to_usize().expect("divider cannot be bigger than usize");
        out.push(match base {
            Base::BASE16 => BASE16_ALPHA[index],
            Base::BASE33 => BASE33_ALPHA[index],
            Base::BASE62 => BASE62_ALPHA[index]
        });
        big /= divider;
    }
    out.chars().rev().collect::<String>()
}

fn base_to_big(string: &str, base: &Base) -> BigUint {
    let chars = string.as_bytes(); // alphabets cannot contain unicode
    let mut big = BigUint::from(0u8);
    let multiplier = &BigUint::from(*base as u64);
    eprintln!("chars {:?}", chars);
    eprintln!("multiplier {:?}", multiplier);
    eprintln!("*BASE33_REV {:?}", *BASE33_REV);
    for i in 0..chars.len() {
        eprintln!("i {:?}", i);
        let index = chars[i] as char;
        eprintln!("index {:?}", index);
        let value = match base {
            Base::BASE16 => BASE16_REV[&index],
            Base::BASE33 => BASE33_REV[&index],
            Base::BASE62 => BASE62_REV[&index]
        };
        eprintln!("value {:?}", value);
        big = big * multiplier + value;
        eprintln!("big {:?}", big);
    }
    big
}



const BASE256_DIGITS: usize = 32;
const BASE16_DIGITS: usize = BASE256_DIGITS * 2;
const BASE62_DIGITS: usize = 43;
const UPPER_BASE62_DIGITS: usize = 22;
const LOWER_BASE62_DIGITS: usize = BASE62_DIGITS - UPPER_BASE62_DIGITS;
const LOWER_BASE33_DIGITS: usize = 25;
lazy_static! {
    static ref PIVOT: BigUint = pow(BigUint::from(62u8), LOWER_BASE62_DIGITS); 
    static ref MAX: BigUint = pow(BigUint::from(256u16), BASE256_DIGITS);
}

#[derive(Clone, Debug)]
struct TTBytes(BigUint);

impl TTBytes {
    pub fn new(big: BigUint) -> TTBytes {
        unimplemented!();
    }

    pub fn base62(&self) -> String {
        big_to_base(self.0.clone(), &Base::BASE62, BASE62_DIGITS)
    }

    pub fn base16(&self) -> String {
        big_to_base(self.0.clone(), &Base::BASE16, BASE16_DIGITS)
    }

    pub fn upper_base62(&self) -> String {
        big_to_base(self.0.clone() / PIVOT.clone(), &Base::BASE62, UPPER_BASE62_DIGITS)
    }

    pub fn lower_base33(&self) -> String {
        big_to_base(self.0.clone() % PIVOT.clone(), &Base::BASE33, LOWER_BASE33_DIGITS)
    }

    pub fn lower_dashed_base33(&self) -> String {
        // TODO: This is less efficient than it might be, because it allocates Results and Vecs.
        return self
            .lower_base33()
            .as_bytes()
            .chunks(5)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join("-")
    }

    pub fn bytes_be(&self) -> Vec<u8> {
        self.0.to_bytes_be()
    }

    pub fn encrypt(&self, buf: &[u8]) -> Vec<u8> {
        unimplemented!();
    }

    pub fn decrypt(&self, buf: &[u8]) -> Vec<u8> {
        unimplemented!();
    }

    pub fn from_bytes_be(u8s: &[u8; 32]) -> TTBytes {
        let big = BigUint::from_bytes_be(u8s);
        eprintln!("from_bytes_be big {:?}", big);
        let ttbytes = TTBytes(big);
        eprintln!("from_bytes_be ttbytes {:?}", &ttbytes);
        ttbytes
    }

    pub fn from_bytes_le(u8s: &[u8; 32]) -> TTBytes {
        unimplemented!();
    }

    pub fn from_base62(base62: &str) -> TTBytes {
        unimplemented!();
    }

    pub fn from_base62_and_base33(base62: &str, base33: &str) -> TTBytes {
        unimplemented!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_base33_to_buffer() {
        assert_eq!(base_to_big("1234567890ABCDEF", &Base::BASE33).to_bytes_be(),
            [13, 127, 141, 10, 8, 5, 125, 141, 84, 24]);
    }

    #[test]
    fn test_convert_buffer_to_base62() {
        assert_eq!(big_to_base(BigUint::from(0u8), &Base::BASE62, BASE62_DIGITS),
            "0000000000000000000000000000000000000000000");
        assert_eq!(big_to_base(BigUint::from(1u8), &Base::BASE62, BASE62_DIGITS),
            "0000000000000000000000000000000000000000001");
        assert_eq!(big_to_base(BigUint::from(61u8), &Base::BASE62, BASE62_DIGITS),
            "000000000000000000000000000000000000000000z");
        assert_eq!(big_to_base(BigUint::from(62u8), &Base::BASE62, BASE62_DIGITS),
            "0000000000000000000000000000000000000000010");
    }

    #[test]
    fn test_convert_buffer_to_base33() {
        assert_eq!(big_to_base(BigUint::from(0u8), &Base::BASE33, 16), "0000000000000000");
        assert_eq!(big_to_base(BigUint::from(1u8), &Base::BASE33, 16), "0000000000000001");
        assert_eq!(big_to_base(BigUint::from(32u8), &Base::BASE33, 16), "000000000000000Z");
        assert_eq!(big_to_base(BigUint::from(33u8), &Base::BASE33, 16), "0000000000000010");
        assert_eq!(big_to_base(BigUint::from_bytes_be(&[13, 127, 141, 10, 8, 5, 125, 141, 84, 24]), &Base::BASE33, 16), "1234567890ABCDEF");
    }

    #[test]
    fn test_it_works() {
        const ARR: [u8; 32] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];
        let ttbytes = &TTBytes::from_bytes_be(&ARR);
        assert_eq!(ttbytes.base62(), "0Eoh211G4c8wtVWM00my5rsNSFlKgaWqQ4mb8gdEqno");
        assert_eq!(ttbytes.upper_base62(), "0Eoh211G4c8wtVWM00my5r");
        assert_eq!(ttbytes.lower_base33(), "DRD7A3JDHFX5A09F1L24SCDVB");
        assert_eq!(ttbytes.lower_dashed_base33(), "DRD7A-3JDHF-X5A09-F1L24-SCDVB");
        assert_eq!(ttbytes.bytes_be(), ARR);
    }
}

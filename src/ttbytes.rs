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

use std::{collections::HashMap, hash::Hash};

use num::{BigUint, ToPrimitive};
//use num::pow::pow;
use lazy_static::lazy_static;

const fn pow(big: BigUint, exp: usize) -> BigUint {
    if exp == 1 {
        return big;
    } else {
        return pow(big, exp - 1);
    }
}


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

struct TTBytes(BigUint);

impl TTBytes {
    pub fn new(big: BigUint) -> TTBytes {
        TTBytes(big % MAX.clone())
    }

    pub fn base62(&self) -> &str {
        ""
    }

    pub fn base16(&self) -> &str {
        ""
    }

    pub fn upper_base62(&self) -> &str {
        ""
    }

    pub fn lower_base33(&self) -> &str {
        ""
    }

    pub fn lower_dashed_base33(&self) -> &str {
        ""
    }

    pub fn bytes(&self) -> [u8; 32] {
        [0u8; 32]
    }

    pub fn encrypt(&self, buf: &[u8]) -> Vec<u8> {
        Vec::new()
    }

    pub fn decrypt(&self, buf: &[u8]) -> Vec<u8> {
        Vec::new()
    }

    pub fn from_bytes(u8s: &[u8; 32]) -> TTBytes {
        TTBytes(BigUint::from_bytes_be(u8s))
    }

    pub fn from_base62(base62: &str) -> TTBytes {
        TTBytes(BigUint::from_bytes_be(&[0u8]))
    }

    pub fn from_base62_and_base33(base62: &str, base33: &str) -> TTBytes {
        TTBytes(BigUint::from_bytes_be(&[0u8]))
    }




}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_base33_to_buffer() {
        assert_eq!(base_to_big("1234567890ABCDEF", &Base::BASE33).to_bytes_be(),
            [13, 127, 141, 10, 8, 5, 125, 141, 84, 24]
        );
    }

    #[test]
    fn test_convert_buffer_to_base33() {
    }

    #[test]
    fn test_it_works() {
        const ARR: [u8; 32] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32];
        let ttbytes = &TTBytes::from_bytes(&ARR);
        assert_eq!(ttbytes.base62(), "0Eoh211G4c8wtVWM00my5rsNSFlKgaWqQ4mb8gdEqno");
        assert_eq!(ttbytes.upper_base62(), "0Eoh211G4c8wtVWM00my5r");
        assert_eq!(ttbytes.lower_base33(), "DRD7A3JDHFX5A09F1L24SCDVB");
        assert_eq!(ttbytes.lower_dashed_base33(), "DRD7A-3JDHF-X5A09-F1L24-SCDVB");
        assert_eq!(ttbytes.bytes(), ARR);
    }
}

/*

describe('ttbytes', function() {
  const arr = new Uint8Array([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32]);

  it('should convert a base33 string into a buffer', function() {
    deepEqual(bigToBuf(baseToBig('1234567890ABCDEF', 33)),
      new Uint8Array([13, 127, 141, 10, 8, 5, 125, 141, 84, 24]));
  });

  it('should convert a buffer into a base33 string', function () {
    deepEqual(bigToBase(bufToBig(new Uint8Array([13, 127, 141, 10, 8, 5, 125, 141, 84, 24])), 33, 16),
      '1234567890ABCDEF');
  });

  it('should work', function () {
    //deepEqual(rev, {});
    const ttbytes = TTBytes.fromUint8Array(arr);
    deepEqual(ttbytes.base62, '0Eoh211G4c8wtVWM00my5rsNSFlKgaWqQ4mb8gdEqno');
    deepEqual(ttbytes.upperAsBase62, '0Eoh211G4c8wtVWM00my5r');
    deepEqual(ttbytes.lowerAsBase33, 'DRD7A3JDHFX5A09F1L24SCDVB');
    deepEqual(ttbytes.lowerAsDashedBase33, 'DRD7A-3JDHF-X5A09-F1L24-SCDVB');
    deepEqual(ttbytes.uint8Array, arr);
  });
});
 */
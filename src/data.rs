use std::fmt::Display;
use std::ops::BitXor;
use std::str;

pub trait BaseRepresentation: Default + Clone {
  fn base(&self) -> u8;
  fn digit_to_ascii(&self, digit: u8) -> u8;
  fn ascii_to_digit(&self, ascii_code: u8) -> u8;

  fn digits_to_ascii(&self, digits: &Vec<u8>) -> Vec<u8> {
    Vec::from_iter(digits.into_iter().map(|x| self.digit_to_ascii(*x)))
  }

  fn ascii_to_digits(&self, ascii_codes: &Vec<u8>) -> Vec<u8> {
    Vec::from_iter(ascii_codes.into_iter().map(|x| self.ascii_to_digit(*x)))
  }
}

#[derive(Clone)]
pub struct StandardBase16 {
  ascii_lookup: Vec<u8>
}

impl Default for StandardBase16 {
  fn default() -> Self { Self { ascii_lookup: Vec::from("0123456789abcdef") } }
}

impl BaseRepresentation for StandardBase16 {
  fn base(&self) -> u8 { 16 }

  fn digit_to_ascii(&self, digit: u8) -> u8 { self.ascii_lookup[digit as usize] }

  fn ascii_to_digit(&self, ascii_code: u8) -> u8 {
    if 48 <= ascii_code && ascii_code <= 57 {
      ascii_code - 48
    } else {
      10 + (ascii_code - 97)
    }
  }
}

#[derive(Clone)]
pub struct StandardBase64 {
  ascii_lookup: Vec<u8>
}

impl Default for StandardBase64 {
  fn default() -> Self {
    Self { ascii_lookup: Vec::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/") }
  }
}

impl BaseRepresentation for StandardBase64 {
  fn base(&self) -> u8 { 64 }

  fn digit_to_ascii(&self, digit: u8) -> u8 { self.ascii_lookup[digit as usize] }

  fn ascii_to_digit(&self, ascii_code: u8) -> u8 {
    if 65 <= ascii_code && ascii_code <= 90 {
      ascii_code - 65
    } else if 97 <= ascii_code && ascii_code <= 122 {
      26 + (ascii_code - 97)
    } else if 48 <= ascii_code && ascii_code <= 57 {
      52 + (ascii_code - 48)
    } else if ascii_code == 43 {
      62
    } else {
      63
    }
  }
}

#[derive(Default, Clone)]
pub struct Data<B: BaseRepresentation> {
  digits: Vec<u8>,
  base_rep: B
}

impl<B: BaseRepresentation> Data<B> {
  pub fn len(&self) -> usize { self.digits.len() }

  pub fn digits(&self) -> &Vec<u8> { &self.digits }
}

impl<B: BaseRepresentation> IntoIterator for Data<B> {
  type Item = u8;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter { self.digits.into_iter() }
}

impl<B: BaseRepresentation> FromIterator<u8> for Data<B> {
  fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self { Self::from(Vec::from_iter(iter)) }
}

impl<B: BaseRepresentation> Into<Vec<u8>> for Data<B> {
  fn into(self) -> Vec<u8> { self.digits }
}

impl<B: BaseRepresentation> From<Vec<u8>> for Data<B> {
  fn from(value: Vec<u8>) -> Self { Self { digits: value, base_rep: B::default() } }
}

impl<B: BaseRepresentation> From<String> for Data<B> {
  fn from(value: String) -> Self { Self::from(B::default().ascii_to_digits(&Vec::from(value))) }
}

impl<B: BaseRepresentation> From<&str> for Data<B> {
  fn from(value: &str) -> Self { Self::from(value.to_string()) }
}

impl<B: BaseRepresentation> BitXor<&Data<B>> for &Data<B> {
  type Output = Data<B>;

  // Challenge 2, Set 1
  fn bitxor(self, rhs: &Data<B>) -> Self::Output {
    let n1 = self.len();
    let n2 = rhs.len();
    if n1 != n2 {
      panic!("Cannot XOR sequences of different lengths ({} and {})", n1, n2);
    }

    let mut res = Vec::with_capacity(n1);

    for i in 0..n1 {
      res.push(self.digits[i] ^ rhs.digits[i]);
    }

    return Data::from(res);
  }
}

impl<B: BaseRepresentation> BitXor<Data<B>> for Data<B> {
  type Output = Data<B>;

  fn bitxor(self, rhs: Data<B>) -> Self::Output { &self ^ &rhs }
}

impl<B: BaseRepresentation> Display for Data<B> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let ascii_codes = self.base_rep.digits_to_ascii(&self.digits);

    let s = match str::from_utf8(&ascii_codes.as_slice()) {
      Ok(v) => v,
      Err(_) => panic!()
    };

    return write!(f, "{}", s);
  }
}

pub type HexData = Data<StandardBase16>;
pub type B64Data = Data<StandardBase64>;

impl HexData {
  // Challenge 1, Set 1
  pub fn to_b64(&self) -> B64Data {
    let n = self.len();
    let mut b64_digits: Vec<u8> = Vec::with_capacity(2 * n / 3);

    let mut acc: u16 = 0;
    let start_count = n % 3;
    let mut count = if start_count == 0 { 3 } else { start_count };
    let pows = vec![1, 16, 256];

    for digit in self.digits() {
      acc += (*digit as u16) * pows[count - 1];
      count -= 1;

      if count == 0 {
        b64_digits.push((acc / 64) as u8);
        b64_digits.push((acc % 64) as u8);

        acc = 0;
        count = 3;
      }
    }

    return B64Data::from(b64_digits);
  }

  pub fn decode(&self) -> String { self.to_b64().to_string() }
}

mod tests {
  use super::*;

  #[test]
  fn test_hex_to_b64() -> Result<(), String> {
    let inputs = vec![
      "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
      "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6",
      "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f",
    ];

    let results = vec![
      "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
      "BJJ20ga2lsbGluZyB5b3VyIGJyYWluIGxpa2UgYSBwb2lzb25vdXMgbXVzaHJvb2",
      "AEknbSBraWxsaW5nIHlvdXIgYnJhaW4gbGlrZSBhIHBvaXNvbm91cyBtdXNocm9v",
    ];

    for i in 0..inputs.len() {
      let res = HexData::from(inputs[i].to_string()).decode();
      if res != results[i] {
        return Err(format!("input {} yields wrong output: {}", i, res));
      };
    }

    return Ok(());
  }

  #[test]
  fn test_hex_xor() -> Result<(), String> {
    let hex1 = HexData::from("1c0111001f010100061a024b53535009181c".to_string());
    let hex2 = HexData::from("686974207468652062756c6c277320657965".to_string());

    let result = "746865206b696420646f6e277420706c6179";
    let res = (hex1 ^ hex2).to_string();

    if res != result {
      return Err(format!("wrong output: {}", res));
    }

    return Ok(());
  }
}

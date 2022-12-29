use hex;
use std::fmt::Display;
use std::ops::BitXor;
use std::str;

//-------------------------------
//   Byte Representation Trait
//-------------------------------

pub trait ByteRepresentation: Default + Clone {
  fn byte_to_ascii(&self, byte: u8) -> u8;

  fn ascii_to_byte(&self, ascii_code: u8) -> u8;

  fn bytes_to_ascii(&self, bytes: &Vec<u8>) -> Vec<u8> {
    Vec::from_iter(bytes.into_iter().map(|x| self.byte_to_ascii(*x)))
  }

  fn ascii_to_bytes(&self, ascii_codes: &Vec<u8>) -> Vec<u8> {
    Vec::from_iter(ascii_codes.into_iter().map(|x| self.ascii_to_byte(*x)))
  }
}

//---------------------------------
//   Base 16 Byte Representation
//---------------------------------

#[derive(Clone)]
pub struct StandardBase16 {
  ascii_lookup: Vec<u8>
}

impl Default for StandardBase16 {
  fn default() -> Self { Self { ascii_lookup: Vec::from("0123456789abcdef") } }
}

impl ByteRepresentation for StandardBase16 {
  fn byte_to_ascii(&self, byte: u8) -> u8 { self.ascii_lookup[byte as usize] }

  fn ascii_to_byte(&self, ascii_code: u8) -> u8 {
    if 48 <= ascii_code && ascii_code <= 57 {
      ascii_code - 48
    } else {
      10 + (ascii_code - 97)
    }
  }
}

//---------------------------------
//   Base 64 Byte Representation
//---------------------------------

#[derive(Clone)]
pub struct StandardBase64 {
  ascii_lookup: Vec<u8>
}

impl Default for StandardBase64 {
  fn default() -> Self {
    Self { ascii_lookup: Vec::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/") }
  }
}

impl ByteRepresentation for StandardBase64 {
  fn byte_to_ascii(&self, byte: u8) -> u8 { self.ascii_lookup[byte as usize] }

  fn ascii_to_byte(&self, ascii_code: u8) -> u8 {
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

//-------------------------------
//   ASCII Byte Representation
//-------------------------------

#[derive(Clone, Default)]
pub struct StandardASCII {}

impl ByteRepresentation for StandardASCII {
  fn byte_to_ascii(&self, byte: u8) -> u8 { byte }

  fn ascii_to_byte(&self, ascii_code: u8) -> u8 { ascii_code }
}

//----------------------
//   Byte Data Struct
//----------------------

#[derive(Default, Clone)]
pub struct Data<B: ByteRepresentation> {
  bytes: Vec<u8>,
  base_rep: B
}

impl<B: ByteRepresentation> Data<B> {
  pub fn len(&self) -> usize { self.bytes.len() }

  pub fn bytes(&self) -> &Vec<u8> { &self.bytes }
}

impl<B: ByteRepresentation> IntoIterator for Data<B> {
  type Item = u8;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter { self.bytes.into_iter() }
}

impl<B: ByteRepresentation> FromIterator<u8> for Data<B> {
  fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self { Self::from(Vec::from_iter(iter)) }
}

impl<B: ByteRepresentation> From<Data<B>> for Vec<u8> {
  fn from(value: Data<B>) -> Self { value.bytes }
}

impl<B: ByteRepresentation> From<Vec<u8>> for Data<B> {
  fn from(value: Vec<u8>) -> Self { Self { bytes: value, base_rep: B::default() } }
}

impl<B: ByteRepresentation> From<String> for Data<B> {
  fn from(value: String) -> Self { Self::from(B::default().ascii_to_bytes(&Vec::from(value))) }
}

impl<B: ByteRepresentation> From<&str> for Data<B> {
  fn from(value: &str) -> Self { Self::from(value.to_string()) }
}

impl<B: ByteRepresentation> BitXor<&Data<B>> for &Data<B> {
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
      res.push(self.bytes[i] ^ rhs.bytes[i]);
    }

    return Data::from(res);
  }
}

impl<B: ByteRepresentation> BitXor<Data<B>> for Data<B> {
  type Output = Data<B>;

  fn bitxor(self, rhs: Data<B>) -> Self::Output { &self ^ &rhs }
}

//-----------------------
//   ASCII Data Struct
//-----------------------

pub type ASCIIData = Data<StandardASCII>;

impl ASCIIData {
  pub fn into<B: ByteRepresentation>(&self) -> Data<B> { Data::from(B::default().ascii_to_bytes(&self.bytes)) }

  // This is not the same as directly converting hexadecimal into ASCII; this is a special ASCII encoding in hex.
  pub fn from_hex_data(value: HexData) -> ASCIIData { ASCIIData::from(hex::decode(value.bytes).unwrap()) }

  // Same goes for this one; this is not equivalent to From<String>() for ASCIIData.
  pub fn from_hex<T: AsRef<[u8]>>(value: T) -> ASCIIData { ASCIIData::from(hex::decode(value).unwrap()) }

  // Same goes for this one; this is not equivalent to to_string().
  pub fn to_hex_string(self) -> String { hex::encode(self.bytes) }
}

impl<B: ByteRepresentation> From<&Data<B>> for ASCIIData {
  fn from(value: &Data<B>) -> Self { Self::from(value.base_rep.bytes_to_ascii(&value.bytes)) }
}

impl<B: ByteRepresentation> Display for Data<B> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let ascii_codes = self.base_rep.bytes_to_ascii(&self.bytes);
    return write!(f, "{}", str::from_utf8(&ascii_codes.as_slice()).unwrap());
  }
}

//-----------------------------
//   Hexadecimal Data Struct
//-----------------------------

pub type HexData = Data<StandardBase16>;

//-------------------------
//   Base 64 Data Struct
//-------------------------

pub type B64Data = Data<StandardBase64>;

impl From<&HexData> for B64Data {
  // Challenge 1, Set 1
  fn from(value: &HexData) -> Self {
    let n = value.len();
    let mut b64_bytes: Vec<u8> = Vec::with_capacity(2 * n / 3);

    let mut acc: u16 = 0;
    let start_count = n % 3;
    let mut count = if start_count == 0 { 3 } else { start_count };
    let pows = vec![1, 16, 256];

    for bytes in value.bytes() {
      acc += (*bytes as u16) * pows[count - 1];
      count -= 1;

      if count == 0 {
        b64_bytes.push((acc / 64) as u8);
        b64_bytes.push((acc % 64) as u8);

        acc = 0;
        count = 3;
      }
    }

    return B64Data::from(b64_bytes);
  }
}

//----------------
//   Unit Tests
//----------------

#[allow(unused_imports)]
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
      let res = B64Data::from(&HexData::from(inputs[i])).to_string();
      if res != results[i] {
        return Err(format!("input {} yields wrong output: {}", i, res));
      };
    }

    return Ok(());
  }

  #[test]
  fn test_hex_xor() -> Result<(), String> {
    let hex1 = super::HexData::from("1c0111001f010100061a024b53535009181c".to_string());
    let hex2 = super::HexData::from("686974207468652062756c6c277320657965".to_string());

    let result = "746865206b696420646f6e277420706c6179";
    let res = (hex1 ^ hex2).to_string();

    if res != result {
      return Err(format!("wrong output: {}", res));
    }

    return Ok(());
  }
}

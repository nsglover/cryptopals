use std::str;

// Set 1, Challenge 1:
pub fn hex_to_b64(hex : &Vec<u8>) -> Vec<u8> {
  let b64_lookup = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes().to_vec();
  let n = hex.len();
  let mut b64 : Vec<u8> = Vec::with_capacity(2 * n / 3);

  let mut acc : u16 = 0;
  let start_count = n % 3;
  let mut count = if start_count == 0 { 3 } else { start_count };
  let pows = vec![1, 16, 256];

  for digit in hex {
    acc += (*digit as u16) * pows[count - 1];
    count -= 1;

    if count == 0 {
      let q = acc / 64;
      b64.push(b64_lookup[q as usize]);
      b64.push(b64_lookup[(acc % 64) as usize]);

      acc = 0;
      count = 3;
    }
  }

  return b64;
}

// Set 1, Challenge 2:
pub fn hex_xor(hex1 : &Vec<u8>, hex2 : &Vec<u8>) -> Vec<u8> {
  let hex_lookup = "0123456789abcdef".as_bytes().to_vec();
  let n = hex1.len();
  let m = hex2.len();
  if n != m {
    panic!("Cannot XOR numbers of different lengths ({} and {})", n, m);
  }

  let mut res = Vec::with_capacity(n);

  for i in 0..n {
    res.push(hex_lookup[(hex1[i] ^ hex2[i]) as usize]);
  }

  return res;
}

// Decodes a hex string into its digits (0 through 15).
pub fn string_to_hex(hex_string : String) -> Vec<u8> {
  let mut bytes = hex_string.as_bytes().to_vec();
  for i in 0..bytes.len() {
    bytes[i] = match u8::from_str_radix(&(bytes[i] as char).to_string(), 16) {
      Ok(v) => v,
      _ => panic!("Invalid HEX code: {}", bytes[i])
    };
  }

  return bytes;
}

// Wrapper function which takes any map from hex digits (0 through 15) to ascii codes as well as a hex string input,
// decodes the string into hex, passes it to the map, and encodes the output as a string.
pub fn ascii_to_string(ascii : &Vec<u8>) -> String {
  return match str::from_utf8(&ascii.as_slice()) {
    Ok(v) => v.to_string(),
    Err(e) => panic!("Invalid ASCII Sequence: {}", e)
  };
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
      let res = ascii_to_string(&hex_to_b64(&string_to_hex(inputs[i].to_string())));
      if res != results[i] {
        return Err(format!("input {} yields wrong output: {}", i, res));
      };
    }

    return Ok(());
  }

  #[test]
  fn test_hex_xor() -> Result<(), String> {
    let hex1 = string_to_hex("1c0111001f010100061a024b53535009181c".to_string());
    let hex2 = string_to_hex("686974207468652062756c6c277320657965".to_string());
    let result = "746865206b696420646f6e277420706c6179";
    let res = ascii_to_string(&hex_xor(&hex1, &hex2));

    if res != result {
      return Err(format!("wrong output: {}", res));
    }

    return Ok(());
  }
}

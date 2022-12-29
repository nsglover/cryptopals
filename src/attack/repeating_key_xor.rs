use crate::data::*;

#[allow(dead_code)]
pub fn encrypt_repeating_key_xor(message: &ASCIIData, key: ASCIIData) -> ASCIIData {
  message ^ &ASCIIData::from_iter(key.into_iter().cycle().take(message.len()))
}

#[allow(unused_imports)]
mod tests {
  use super::*;
  use crate::data::ASCIIData;

  #[test]
  fn test_encrypt_repeating_key_xor() -> Result<(), String> {
    let message = ASCIIData::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    let key = ASCIIData::from("ICE");

    let result = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272".to_string()
      + "a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
    let res = encrypt_repeating_key_xor(&message, key).to_hex_string();

    if res != result {
      return Err(format!("wrong output: {}", res));
    }

    return Ok(());
  }
}

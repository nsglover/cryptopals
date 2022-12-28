use crate::data::*;
use std::cmp;

const ENGLISH_CHAR_FREQUENCIES: [f32; 27] = [
  0.0653, 0.0126, 0.0223, 0.0328, 0.1027, 0.0198, 0.0162, 0.0498, 0.0567, 0.0010, 0.0056, 0.0332, 0.0203, 0.0517,
  0.0616, 0.0150, 0.0008, 0.0499, 0.0532, 0.0752, 0.0228, 0.0080, 0.0170, 0.0014, 0.0143, 0.0005, 0.1823
];

pub fn freq_and_alphabet_score(data: &ASCIIData) -> (f32, f32) {
  fn uppercase(b: usize) -> bool { 65 <= b && b <= 90 }
  fn lowercase(b: usize) -> bool { 97 <= b && b <= 122 }

  let mut counts = vec![0u64; 256];

  for &b in data.bytes() {
    counts[b as usize] += 1
  }

  let mut norm_squared = 0.0;
  let mut num_letters = 0;

  for i in 0..256 {
    let mut diff = counts[i] as f32;
    if uppercase(i) || lowercase(i) || i == 32 {
      let j = if uppercase(i) {
        i - 65
      } else if i == 32 {
        26
      } else {
        i - 97
      };

      num_letters += counts[i];
      diff -= ENGLISH_CHAR_FREQUENCIES[j] * (data.len() as f32);
    }

    norm_squared += diff * diff;
  }

  return (norm_squared, (num_letters as f32) / (data.len() as f32));
}

pub fn attack_single_byte_xor(ciphertext: &ASCIIData) -> (u8, (f32, f32), ASCIIData) {
  let n = ciphertext.len();
  let build_key = |character| ASCIIData::from(vec![character; n]);
  let argmax = (0..256u16)
    .into_iter()
    .zip(0..256u16)
    .map(|(arg, i)| (arg, ciphertext ^ &build_key(i as u8)))
    .map(|(arg, msg)| (arg as u8, freq_and_alphabet_score(&msg), msg))
    .filter(|(_, (_, validity), _)| *validity > 0.7)
    .min_by(|(_, (s1, _), _), (_, (s2, _), _)| s1.partial_cmp(s2).unwrap_or(cmp::Ordering::Equal));

  return argmax.unwrap();
}

mod tests {
  // Challenge 3, Set 1
  #[test]
  fn test_attack() -> Result<(), String> {
    let ciphertext = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let (key, _, msg) = super::attack_single_byte_xor(&crate::data::ASCIIData::from_hex(ciphertext));
    if key != 88 || msg.to_string() != "Cooking MC's like a pound of bacon".to_string() {
      return Err(format!("Attack failed; key was {} with message {}", key, msg));
    }

    return Ok(());
  }
}

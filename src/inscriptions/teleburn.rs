use {super::*, sha3::Digest, sha3::Keccak256};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Ethereum(String);

impl From<InscriptionId> for Ethereum {
  fn from(inscription_id: InscriptionId) -> Self {
    let mut array = [0; 36];
    let (txid, index) = array.split_at_mut(32);
    txid.copy_from_slice(inscription_id.txid.as_ref());
    index.copy_from_slice(&inscription_id.index.to_be_bytes());
    let digest = bitcoin::hashes::sha256::Hash::hash(&array);
    Self(create_address_with_checksum(&hex::encode(&digest[0..20])))
  }
}

impl Display for Ethereum {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

/// Given the hex digits of an Ethereum address, return that address with a
/// checksum as per https://eips.ethereum.org/EIPS/eip-55
fn create_address_with_checksum(address: &str) -> String {
  assert_eq!(address.len(), 40);
  assert!(address
    .chars()
    .all(|c| c.is_ascii_hexdigit() && (!c.is_alphabetic() || c.is_lowercase())));

  let hash = hex::encode(&Keccak256::digest(address.as_bytes())[..20]);
  assert_eq!(hash.len(), 40);

  "0x"
    .chars()
    .chain(address.chars().zip(hash.chars()).map(|(a, h)| match h {
      '0'..='7' => a,
      '8'..='9' | 'a'..='f' => a.to_ascii_uppercase(),
      _ => unreachable!(),
    }))
    .collect()
}

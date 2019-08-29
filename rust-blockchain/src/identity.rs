//copy from ark ecosystem
use hex;
use secp256k1::{All, Error, Message, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

lazy_static! {
    pub static ref SECP256K1: Secp256k1<All> = Secp256k1::new();
}
//private key
pub type PrivateKey = SecretKey;

pub fn privatekey_from_passphrase(passphrase: &str) -> PrivateKey {
    PrivateKey::from_slice(&Sha256::digest(passphrase.as_bytes())[..]).unwrap()
}

pub fn privatekey_to_signature(bytes: &[u8], passphrase: &str) -> String {
    let key = privatekey_from_passphrase(passphrase);
    let hash = &Sha256::digest(&bytes);
    let msg = Message::from_slice(&hash).unwrap();
    let sig = SECP256K1.sign(&msg, &key);

    hex::encode(sig.serialize_der())
}
//public key
pub fn publickkey_from_passphrase(passphrase: &str) -> Result<PublicKey, Error> {
    let private_key = privatekey_from_passphrase(passphrase);
    Ok(PublicKey::from_secret_key(&SECP256K1, &private_key))
}

pub fn publickey_from_private_key(private_key: &PrivateKey) -> PublicKey {
    PublicKey::from_secret_key(&SECP256K1, private_key)
}

//address
pub fn address_from_public_key(public_key: &PublicKey, network_version: Option<u8>) -> String {
    let network_version = match network_version {
        Some(network_version) => network_version,
        None => 0,
    };
    "".to_owned()
}
pub fn address_from_passphrase(passphrase: &str, network_version: Option<u8>) -> String {
    let private_key = privatekey_from_passphrase(passphrase);
    address_from_private_key(&private_key, network_version)
}
pub fn address_from_private_key(private_key: &PrivateKey, network_version: Option<u8>) -> String {
    let public_key = publickey_from_private_key(private_key);
    address_from_public_key(&public_key, network_version)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn private_key_from_passphrase() {
        let private_key = privatekey_from_passphrase("this is a top secret passphrase");
        assert_eq!(
            private_key.to_string(),
            "d8839c2432bfd0a67ef10a804ba991eabba19f154a3d707917681d45822a5712"
        );
    }
    #[test]
    fn public_key_from_passphrase() {
        let public_key = publickkey_from_passphrase("this is a top secret passphrase");
        assert_eq!(
            public_key.unwrap().to_string(),
            "034151a3ec46b5670a682b0a63394f863587d1bc97483b1b6c70eb58e7f0aed192"
        );
    }
}

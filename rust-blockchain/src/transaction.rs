use bincode::{deserialize, serialize};
use identity::*;
use secp256k1::Signature;
use sha2::{Digest, Sha256};
use bs58;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct transaction {
    pub from: String,
    pub to: String,
    pub value: Vec<u8>,
    pub sender_public_key: String,
    pub signature: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct transaction_module {
    current: Vec<transaction>,
}

impl transaction_module {
    pub fn new() -> Self {
        transaction_module { current: vec![] }
    }

    pub fn create_and_broadcast_transaction(&mut self, from: String, to: String) -> Result<(), ()> {
        let mut transac = transaction::new();
        transac.from = from;
        transac.to = to;

        //todo verify transaction

        self.current.push(transac);

        //todo broadcast

        Ok(())
    }

    pub fn receive_transaction(&mut self, transac: &transaction) {
        self.current.push(transac.clone());
    }

    pub fn list_transaction_local(&self) {
        println!("list_transaction_local:");
        for x in self.current.iter() {
            println!("{:?}", x);
        }
    }
    pub fn get_current(&self) -> &Vec<transaction> {
        &self.current
    }
}
impl transaction {
    fn new() -> Self {
        transaction {
            ..Default::default()
        }
    }
    pub fn sign(&mut self, passphrase: &str) -> &Self {
        let private_key = privatekey_from_passphrase(passphrase);
        let public_key = publickey_from_private_key(&private_key);
        self.sender_public_key = public_key.to_string();
        self.signature = privatekey_to_signature(self.value.as_slice(), passphrase);
        self
    }
    fn internal_verify(&self, sender_public_key: &str, signature: &str, bytes: &[u8]) -> bool {
        let hash = Sha256::digest(&bytes);
        let msg = secp256k1::Message::from_slice(&hash).unwrap();

        let sig = Signature::from_der(&hex::decode(signature).unwrap()).unwrap();
        let pk = publickey_from_hex(&sender_public_key);
        
        SECP256K1.verify(&msg, &sig, &pk).is_ok()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![];

        return buffer;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::identity;
    #[test]
    fn sign_and_verify() {
        let bytes:Vec<u8> = vec![1,2,3,4,5];
        let passphrase = "this is a passphrase";

        let mut transac = transaction::new();
        transac.value = bytes;
        
        transac.sign(passphrase);
        transac.sender_public_key = identity::publickkey_from_passphrase(passphrase).to_string();
        
        let result = transac.internal_verify(&transac.sender_public_key, &transac.signature, transac.value.as_slice());
        assert!(result);
        let result = transac.internal_verify(&transac.sender_public_key, &transac.signature, &[1,2]);
        assert!(!result);
    }
}

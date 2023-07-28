use hex;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

fn main() {

    let message = "hello";
    let signature = "5c0e32248c10f7125b32cae1de9988f2dab686031083302f85b0a82f78e9206516b272fb7641f3e8ab63cf9f3a9b9220b2d6ff2699dc34f0d000d7693ca1ea5e1c";
    let eth_address = "c9B28DCA7ea6c5e176a58Ba9dF53C30bA52c6642";
    
    let secp = Secp256k1::new();

    let signature = Signature::from_compact(&hex::decode(&signature).unwrap()).unwrap();

    let public_key = PublicKey::from_slice(&hex::decode(&eth_address).unwrap()).unwrap();

    let message = Message::from_slice(&message.as_bytes()).unwrap();

    secp.verify_ecdsa(&message, &signature, &public_key).unwrap();

}

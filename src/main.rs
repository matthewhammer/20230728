use hex;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

fn main() {

    let message = "hello";
    let signature =
	"abb099ec179f6ae0575eea32639a7e842810902d2d97beeedb50dfee63e2169773c49f7e055f8ce12e354e7a82a057f867690a494159652445164a867d5ffd1a1b";

    /* "5c0e32248c10f7125b32cae1de9988f2dab686031083302f85b0a82f78e9206516b272fb7641f3e8ab63cf9f3a9b9220b2d6ff2699dc34f0d000d7693ca1ea5e1c"; */
    
    let eth_address = "c9B28DCA7ea6c5e176a58Ba9dF53C30bA52c6642";
    
    let secp = Secp256k1::new();

    println!("hello world {}", hex::decode(&signature).unwrap().len());

    

    let s1 = hex::decode(&signature).unwrap();
    let s2 = &s1[0..s1.len() - 1];
    
    println!("hello world {}", s2.len());

    let signature = Signature::from_compact(s2).unwrap();

    let public_key = PublicKey::from_slice(&hex::decode(&eth_address).unwrap()).unwrap();

    let message = Message::from_slice(&message.as_bytes()).unwrap();

    secp.verify_ecdsa(&message, &signature, &public_key).unwrap();

}

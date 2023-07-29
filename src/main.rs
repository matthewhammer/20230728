use ethers_core::types::{Address, RecoveryMessage, Signature};
use std::str::FromStr;

fn main() {
    let eth_address = "c9B28DCA7ea6c5e176a58Ba9dF53C30bA52c6642";
    let message = "hello";
    let signature =
	"5c0e32248c10f7125b32cae1de9988f2dab686031083302f85b0a82f78e9206516b272fb7641f3e8ab63cf9f3a9b9220b2d6ff2699dc34f0d000d7693ca1ea5e1c";

    let valid = Signature::from_str(signature)
        .unwrap()
        .verify(
            RecoveryMessage::Data(message.bytes().collect()),
            Address::from_str(eth_address).unwrap(),
        )
        .is_ok();

    println!("valid = {valid}");
}

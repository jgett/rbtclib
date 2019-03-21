extern crate rand;
extern crate secp256k1;
extern crate bitcoin;
 
use bitcoin::network::constants::Network;
use bitcoin::util::address::Address;
use bitcoin::util::key;
use secp256k1::Secp256k1;
use rand::thread_rng;

#[no_mangle]
pub extern fn generate_random_address() -> String {
    // Generate random key pair
    let s = Secp256k1::new();
    let public_key = key::PublicKey {
        compressed: true,
        key: s.generate_keypair(&mut thread_rng()).1,
    };
 
    // Generate pay-to-pubkey-hash address
    let address = Address::p2pkh(&public_key, Network::Bitcoin);
    return address.to_string();
    //println!("{}", address);
}
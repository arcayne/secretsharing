#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate bitcoin;
extern crate crypto;
pub mod account;
pub mod coins;
pub mod context;
pub mod error;
pub mod mnemonic;
pub mod proved;
pub mod sss;

use bitcoin::network::constants::Network;
use bitcoin::Address;
use rand::{thread_rng, Rng};
use account::{Account, AccountAddressType, MasterAccount, MasterKeyEntropy, Seed, Unlocker};
use sss::{ShamirSecretSharing, Share};

const PASSPHRASE: &str = "correct horse battery staple";

fn main() {
    println!("Entered main");


    // create an new random account        
    let master = MasterAccount::new(MasterKeyEntropy::Sufficient, Network::Bitcoin, PASSPHRASE).unwrap();
    println!("master public {}", master.master_public());

    // extract seed
    let seed = master.seed(Network::Bitcoin, PASSPHRASE).unwrap();
    //println!("seed {}", seed);
    // cut seed into 5 shares such that any 3 of them is sufficient to re-construct
    let shares = ShamirSecretSharing::generate(1, &[(3,5)], &seed, None, 1).unwrap();

    //Show all the keys

    for share in shares.iter() {
               println!("Share is {}", share.id);
               println!("Share is {}", share.group_count);
    }

    // re-construct seed from the first 3
    let reconstructed_seed = ShamirSecretSharing::combine(&shares[..3], None).unwrap();

    // re-construct master from seed
    let reconstructed_master = MasterAccount::from_seed(&reconstructed_seed, 0, Network::Bitcoin, PASSPHRASE).unwrap();


    println!("Exit main");
}

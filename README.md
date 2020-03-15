Started from:
https://github.com/rust-bitcoin/rust-wallet

Added main to start testing capabilities.

Get started:

cargo build
cargo run

```
## Shamir's Secret Shares
```rust
// create an new random account        
let master = MasterAccount::new(MasterKeyEntropy::Low, Network::Bitcoin, PASSPHRASE).unwrap();

// extract seed
let seed = master.seed(Network::Bitcoin, PASSPHRASE).unwrap();

// cut seed into 5 shares such that any 3 of them is sufficient to re-construct
let shares = ShamirSecretSharing::generate(1, &[(3,5)], &seed, None, 1).unwrap();

// re-construct seed from the first 3
let reconstructed_seed = ShamirSecretSharing::combine(&shares[..3], None).unwrap();

// re-construct master from seed
let reconstructed_master = MasterAccount::from_seed(&reconstructed_seed, 0, Network::Bitcoin, PASSPHRASE).unwrap();

// prove that everything went fine
assert_eq!(master.master_public(), reconstructed_master.master_public());
assert_eq!(master.encrypted(), reconstructed_master.encrypted());
```
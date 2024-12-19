use bitcoin_wallet::mnemonic::language::LANG;
use bitcoin_wallet::mnemonic::BIP39;

fn main() {
    BIP39::new(128, LANG::ENG).mnemonic();
    BIP39::new(128, LANG::JP).mnemonic();
    BIP39::new(128, LANG::FR).mnemonic();
    BIP39::new(128, LANG::KO).mnemonic();
    BIP39::new(128, LANG::SP).mnemonic();
    BIP39::new(128, LANG::PT).mnemonic();
}

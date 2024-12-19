use bitcoin_wallet::mnemonic::language::LANG;
use bitcoin_wallet::mnemonic::BIP39;

fn main() {
    let i = BIP39::new(128, LANG::ENG);
    println!("{:?}", &i);
    i.mnemonic()

}

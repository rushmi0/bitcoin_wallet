use bitcoin_wallet::mnemonic::language::LANG;
use bitcoin_wallet::mnemonic::secret_word::SecretWord;

fn main() {
    println!("Hello, world!");

    let i = SecretWord::new(1111, LANG::ENG);
    println!("{:?}", &i);
    i.entropy(256);


}

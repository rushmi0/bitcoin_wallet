use bitcoin_wallet::mnemonic::language::LANG;
use bitcoin_wallet::mnemonic::secret_word::SecretWord;

fn main() {
    let i = SecretWord::new(256, LANG::ENG);
    println!("{:?}", &i);
    i.entropy();
}

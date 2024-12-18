use bitcoin_wallet::mnemonic::language::LANG;
use bitcoin_wallet::mnemonic::secret_word::SecretWord;

fn main() {
    let i = SecretWord::new(128, LANG::JP);
    println!("{:?}", &i);
    i.display();

}

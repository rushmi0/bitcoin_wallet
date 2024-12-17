use crate::mnemonic::language::english::WORD_LIST;

pub fn print_words() {
    for (index, word) in WORD_LIST.iter().enumerate() {
        println!("Index {}: {}", index, word);
    }
}

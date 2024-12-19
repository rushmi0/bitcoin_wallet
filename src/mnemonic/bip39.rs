use std::str::from_utf8;
use rand::random;
use sha2::{Sha256, Digest};
use unicode_normalization::UnicodeNormalization;
use crate::mnemonic::language::{
    LANG,
    ENG_WORD_LIST,
    JP_WORD_LIST,
    FR_WORD_LIST,
    KO_WORD_LIST
};

#[derive(Debug)]
pub struct BIP39 {
    strength: usize,
    language: LANG,
}

impl BIP39 {

    pub fn new(strength: usize, language: LANG) -> Self {
        Self {
            strength,
            language
        }
    }

    fn load_wordlist(&self) -> Vec<String> {
        let word_list: &[&str; 2048] = match self.language {
            LANG::ENG => &ENG_WORD_LIST,
            LANG::JP => &JP_WORD_LIST,
            LANG::FR => &FR_WORD_LIST,
            LANG::KO => &KO_WORD_LIST
        };

        word_list.iter().map(|&word| word.nfc().collect::<String>()).collect()
    }

    pub fn entropy(&self) -> String {
        // คำนวณจำนวนไบต์ที่ต้องใช้เพื่อสร้างข้อมูลสุ่ม
        // แปลงจำนวนบิต (self.strength) ให้เป็นจำนวนไบต์ โดยใช้สูตร (strength + 7) / 8
        // เช่น หาก strength = 15 จะต้องการ 2 ไบต์ (เพราะ 15 บิตเกินความจุของ 1 ไบต์ ซึ่งมี 8 บิต)
        let byte_length = (self.strength + 7) / 8;

        // สร้าง buffer ซึ่งเป็นตัวเก็บข้อมูลสุ่มแบบไบต์
        // เริ่มต้นเป็นเวกเตอร์ขนาด byte_length และเติมค่าเริ่มต้นเป็น 0
        let mut random_bytes: Vec<u8> = vec![0; byte_length];

        // เติมค่าตัวเลขสุ่มใน buffer ทีละไบต์
        // ใช้ฟังก์ชัน random::<u8>() เพื่อสุ่มค่า 8 บิต (1 ไบต์)
        for byte in random_bytes.iter_mut() {
            *byte = random::<u8>();
        }

        // แปลงข้อมูลใน buffer (random_bytes) เป็นสตริงของตัวเลขฐานสอง (binary string)
        // โดยแต่ละไบต์จะถูกแปลงเป็นสตริงความยาว 8 บิต
        let mut binary_string = random_bytes
            .iter() // เข้าถึงแต่ละไบต์ในเวกเตอร์
            .map(|byte| format!("{:08b}", byte)) // แปลงไบต์แต่ละตัวเป็นสตริงฐานสอง 8 บิต
            .collect::<String>(); // รวบรวมสตริงทั้งหมดให้เป็นสตริงใหญ่เพียงหนึ่งเดียว

        // ตัดแต่งความยาวของ binary_string ให้เหลือตรงตามจำนวนบิตที่กำหนดใน strength
        // เช่น หาก binary_string มีความยาวเกิน strength (เช่น 16 บิต แต่ต้องการเพียง 10 บิต)
        // จะตัดให้เหลือเพียง 10 ตัวอักษรแรก
        if binary_string.len() > self.strength {
            binary_string.truncate(self.strength);
        }

        binary_string
    }


    fn binary_checksum(&self, binary: &String) -> String {
        let bytes = binary
            .as_bytes()
            .chunks(8)
            .map(|chunk| u8::from_str_radix(from_utf8(chunk).unwrap(), 2).unwrap())
            .collect::<Vec<u8>>();

        let size = bytes.len() * 8;

        let entropy_hash = Sha256::digest(&bytes).to_vec();

        let hash_binary = entropy_hash
            .iter()
            .map(|byte| format!("{:08b}", byte))
            .collect::<String>();

        let checksum_size = size / 32;
        let checksum = &hash_binary[0..checksum_size];

        checksum.to_string()
    }

    pub fn mnemonic(&self) {
        let entropy = self.entropy();
        let checksum = self.binary_checksum(&entropy);

        // รวมค่า entropy และ checksum เป็น raw_binary
        let raw_binary = entropy.clone() + &checksum;

        // ตัด raw_binary ออกเป็นชิ้นย่อยขนาด 11 ตัวอักษร
        let chunks: Vec<&str> = raw_binary
            .as_bytes()
            .chunks(11)
            .map(|chunk| from_utf8(chunk).unwrap())
            .collect();

        // แปลงแต่ละชิ้นจาก string binary เป็นเลขฐาน 10
        let word_indexes: Vec<u16> = chunks
            .iter()
            .map(|chunk| u16::from_str_radix(chunk, 2).unwrap())
            .collect();

        // แมป word_indexes ไปเป็นคำใน wordlist
        let mnemonic_words: Vec<String> = word_indexes
            .iter()
            .map(|&index| self.load_wordlist()[index as usize].clone())
            .collect();


        println!("Entropy: {}", &entropy);
        println!("Raw binary chunks: {:?}", chunks);
        println!("Decimal values: {:?}", word_indexes);
        println!("Mnemonic phrase: {:?}", mnemonic_words.join(", "));
    }


}

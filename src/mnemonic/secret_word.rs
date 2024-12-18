
use crate::mnemonic::language::lang::LANG;
use rand::random;
use crate::mnemonic::language::{ENG_WORD_LIST, JP_WORD_LIST};

#[derive(Debug)]
pub struct SecretWord {
    strength: usize,
    language: LANG,
}

impl SecretWord {

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
        };

        word_list.iter().map(|&word| word.to_string()).collect()
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

        // คืนค่าสตริงฐานสองที่ได้
        binary_string
    }

    pub fn display(&self) {
        println!("{:?}", self.load_wordlist())
    }
}

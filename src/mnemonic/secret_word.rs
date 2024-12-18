use crate::mnemonic::language::english::WORD_LIST;
use crate::mnemonic::language::lang::LANG;
use rand::random;

#[derive(Debug)]
pub struct SecretWord {
    strength: usize,
    language: LANG,
}

impl SecretWord {

    pub fn new(strength: usize, language: LANG) -> Self {
        Self { strength, language }
    }

    fn load_wordlist(&self) -> Vec<String> {
        WORD_LIST.iter().map(|&word| word.to_string()).collect()
    }

    pub fn entropy(&self) {
        // คำนวณจำนวนไบต์ (byte) ที่ต้องการใช้ในการเก็บข้อมูลสุ่ม
        // แปลงจาก self.strength (bit_length) เป็น byte_length โดยใช้ (strength + 7) / 8
        // เช่น ถ้า strength = 15 จะได้ byte_length = 2 เพราะ 15 บิต ต้องใช้ 2 ไบต์ (8 บิตต่อไบต์)
        let byte_length = (self.strength + 7) / 8;

        // สร้าง buffer (ตัวแปรเก็บข้อมูล) สำหรับเก็บค่าไบต์แบบสุ่ม
        // เริ่มต้นเป็นเวกเตอร์ขนาด byte_length ทั้งหมดมีค่าเริ่มต้นเป็น 0
        let mut random_bytes: Vec<u8> = vec![0; byte_length];

        // เติมค่าใน buffer ด้วยตัวเลขสุ่ม (สุ่มทีละไบต์)
        // ใช้ฟังก์ชัน random::<u8>() เพื่อสุ่มค่า 8 บิต (u8)
        for byte in random_bytes.iter_mut() {
            *byte = random::<u8>(); // เขียนค่าที่สุ่มได้ลงใน buffer แต่ละไบต์
        }

        // แปลงค่าที่อยู่ใน buffer (random_bytes) ให้เป็นสตริงที่เป็นฐานสอง (binary string)
        // แต่ละไบต์ถูกแปลงให้เป็นสตริงฐานสองที่มีความยาว 8 บิต
        let mut binary_string = random_bytes
            .iter() // เข้าถึงแต่ละไบต์ใน buffer
            .map(|byte| format!("{:08b}", byte)) // แปลงไบต์แต่ละตัวไปเป็น binary string ความยาว 8 บิต
            .collect::<String>(); // รวมค่า binary string ทุกตัวเข้าด้วยกันให้กลายเป็นสตริงใหญ่

        // ตัดสตริงฐานสองให้มีความยาวตรงตามจำนวน strength (จำนวนบิตตามที่กำหนด)
        // ตัวอย่าง: ถ้าสร้าง binary string ความยาว 16 บิต แต่กำหนด strength = 10
        // เราจะตัดให้เหลือเพียง 10 บิตแรก
        if binary_string.len() > self.strength {
            binary_string.truncate(self.strength); // ลดความยาวของ binary_string ให้เท่ากับ strength
        }

        // แสดงผลลัพธ์
        // - สตริงฐานสองที่ได้จากการสุ่ม
        // - จำนวนบิตในสตริง (เท่ากับ self.strength เสมอเมื่อทำการ truncate แล้ว)
        println!("Binary String: {} | Length: {}", binary_string, binary_string.len());
    }

    pub fn display(&self) {
        println!("{:?}", self.load_wordlist())
    }
}

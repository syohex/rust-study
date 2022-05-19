mod vigenere {
    pub fn encrypt(plaintext: &str, key: &str) -> String {
        let mut ret = String::new();
        let kb: Vec<u8> = key.bytes().collect();

        let key_len = kb.len();
        for (i, b) in plaintext.bytes().enumerate() {
            let b = b - b'A';
            let k = kb[i % key_len] - b'A';
            let p = (b + k) % 26;
            ret.push((p + b'A') as char);
        }

        ret
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let mut ret = String::new();
        let kb: Vec<u8> = key.bytes().map(|b| b - b'A').collect();

        let key_len = kb.len();
        for (i, b) in ciphertext
            .bytes()
            .filter(|b| *b >= b'A' && *b <= b'Z')
            .map(|b| b - b'A')
            .enumerate()
        {
            let p = (26 + b - kb[i % key_len]) % 26;
            ret.push((p + b'A') as char);
        }

        ret
    }
}

fn main() {
    {
        let key = "WHYRUST";
        let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
        let plaintext = vigenere::decrypt(&ciphertext, key);
        println!("{}", plaintext);
    }

    {
        let key = "ARM";
        let plain = "CODE";
        let encoded = vigenere::encrypt(plain, key);
        println!("CODE(ARM) => {encoded}");
    }
}

#[test]
fn test_decrypt() {
    {
        let key = "ARM";
        let ciphertext = "CFPE";
        let plain = vigenere::decrypt(&ciphertext, key);
        assert_eq!(plain, "CODE");
    }
}

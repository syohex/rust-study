const LOOKUP_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

const PADDING: char = '=';

fn encode_chunks(chunks: &[u8]) -> Vec<char> {
    let mut v = Vec::new();
    match chunks.len() {
        3 => {
            let b1 = chunks[0] >> 2;
            v.push(LOOKUP_TABLE[b1 as usize]);

            let b2 = ((chunks[0] & 0b00000011) << 4) | (chunks[1] >> 4);
            v.push(LOOKUP_TABLE[b2 as usize]);

            let b3 = ((chunks[1] & 0b00001111) << 2) | ((chunks[2] & 0b11000000) >> 6);
            v.push(LOOKUP_TABLE[b3 as usize]);

            let b4 = chunks[2] & 0b00111111;
            v.push(LOOKUP_TABLE[b4 as usize]);
        }
        2 => {
            let b1 = chunks[0] >> 2;
            v.push(LOOKUP_TABLE[b1 as usize]);

            let b2 = ((chunks[0] & 0b00000011) << 4) | (chunks[1] >> 4);
            v.push(LOOKUP_TABLE[b2 as usize]);

            let b3 = (chunks[1] & 0b00001111) << 2;
            v.push(LOOKUP_TABLE[b3 as usize]);
            v.push(PADDING);
        }
        1 => {
            let b1 = chunks[0] >> 2;
            v.push(LOOKUP_TABLE[b1 as usize]);

            let b2 = (chunks[0] & 0b00000011) << 4;
            v.push(LOOKUP_TABLE[b2 as usize]);
            v.push(PADDING);
            v.push(PADDING);
        }
        _ => {}
    }

    v
}

pub fn base64(data: &str) -> String {
    let byte_array: &[u8] = data.as_bytes();
    let mut v: Vec<char> = Vec::new();

    for octet_array in byte_array.chunks(3) {
        v.extend(encode_chunks(octet_array));
    }

    return v.into_iter().collect::<String>();
}

fn decode_string(input: &str) -> Vec<u8> {
    let filtered = input
        .chars()
        .map(|c| if c == PADDING { 'A' } else { c })
        .collect::<Vec<char>>();
    let equals = input.chars().filter(|&c| c == '=').count();
    let mut v: Vec<u8> = Vec::new();

    for chunk in filtered.chunks(4) {
        let mut n = LOOKUP_TABLE.iter().position(|&x| x == chunk[0]).unwrap() << 18;
        let n1 = LOOKUP_TABLE.iter().position(|&x| x == chunk[1]).unwrap() << 12;
        let n2 = LOOKUP_TABLE.iter().position(|&x| x == chunk[2]).unwrap() << 6;
        let n3 = LOOKUP_TABLE.iter().position(|&x| x == chunk[3]).unwrap();

        n = n | n1 | n2 | n3;
        v.push(((n >> 16) & 0xff) as u8);
        v.push(((n >> 8) & 0xff) as u8);
        v.push((n & 0xff) as u8);
    }

    let len = v.len() - equals;
    v.into_iter().take(len).collect()
}

pub fn base64_decode(data: &str) -> String {
    if data.len() % 4 != 0 {
        String::from("Incompatible String")
    } else {
        std::str::from_utf8(&decode_string(data))
            .unwrap()
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(base64("hello world"), "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_decode() {
        assert_eq!(base64_decode("aGVsbG8gd29ybGQ="), "hello world");
    }
}

use cryptography::Cipher;

pub struct RC5 {
    key: Vec<u8>,
    rounds: usize,
    word_size: usize,
    number_of_rounds: usize,
    key_size: usize,
}

impl RC5 {
    pub fn new(key: Vec<u8>, rounds: usize, word_size: usize) -> Self {
        if word_size % 16 != 0 {
            panic!("Invalid word size");
        }

        let number_of_rounds = 2 * rounds + 2;
        let key_size = key.len();
        Self {
            key,
            rounds,
            word_size,
            number_of_rounds,
            key_size,
        }
    }

    fn destroy(&mut self) {
        self.key.clear();
        self.rounds = 0;
        self.word_size = 0;
        self.number_of_rounds = 0;
        self.key_size = 0;
    }
}

impl Cipher for RC5 {
    fn encrypt(&mut self, input: &[u8]) -> Vec<u8> {
        Vec::new()
    }

    fn decrypt(&mut self, input: &[u8]) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc5() {
        let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let mut rc5 = RC5::new(key, 12, 64);
        let input = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let output = rc5.encrypt(&input);
        assert_eq!(output, vec![0x8d, 0x6a, 0x97, 0x6d]);
    }
}

use cryptography::{galois_multiplication, AdvancedEncryptionStandard, Cipher};

static SBOX: [[u8; 16]; 16] = 
[
    [0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76],
    [0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0],
    [0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15],
    [0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75],
    [0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84],
    [0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf],
    [0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8],
    [0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2],
    [0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73],
    [0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb],
    [0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79],
    [0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08],
    [0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a],
    [0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e],
    [0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf],
    [0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16],
];

pub struct AdvancedEncryptionStandard128Bit {
    key: [[u8; 4]; 48]
}

impl AdvancedEncryptionStandard for AdvancedEncryptionStandard128Bit {
    fn add_round_key(&mut self, state: &mut [[u8; 4]; 4], round: &mut usize) {
        let round_key:&[[u8; 4]] = &self.key[*round * 4..(*round + 1) * 4];

        for (i, item) in state.iter_mut().enumerate().take(4) {
            for j in 0..4 {
                item[j] ^= round_key[j][i];
            }
        }

        *round += 1;
    }

    fn mix_columns(&mut self, state: &mut [[u8; 4]; 4]) {
        let mut temp = [[0u8; 4]; 4];

        for c in 0..4 {
            temp[0][c] = galois_multiplication(0x02, state[0][c]) ^ galois_multiplication(0x03, state[1][c]) ^ state[2][c] ^ state[3][c];
            temp[1][c] = state[0][c] ^ galois_multiplication(0x02, state[1][c]) ^ galois_multiplication(0x03, state[2][c]) ^ state[3][c];
            temp[2][c] = state[0][c] ^ state[1][c] ^ galois_multiplication(0x02, state[2][c]) ^ galois_multiplication(0x03, state[3][c]);
            temp[3][c] = galois_multiplication(0x03, state[0][c]) ^ state[1][c] ^ state[2][c] ^ galois_multiplication(0x02, state[3][c]);
        }

        state.copy_from_slice(&temp);
    }

    fn shift_rows(&mut self, state: &mut [[u8; 4]; 4]) {
        for (i, item) in state.iter_mut().enumerate().take(4).skip(1) {
            let mut temp = [0u8; 4];
            for ii in 0..4 {
                temp[ii] = item[(i + ii) % 4];
            }

            item.copy_from_slice(&temp);
        }
    }

    fn sub_bytes(&mut self, state: &mut [[u8; 4]; 4]) {
        for item in state.iter_mut().take(4) {
            for item in item.iter_mut().take(4) {
                let byte = *item;
                
                let row = (byte >> 4) as usize; 
                let col = (byte & 0x0F) as usize; 
                
                *item = SBOX[row][col]; 
            }
        }
    }
    
}


impl Cipher for AdvancedEncryptionStandard128Bit {
    fn encrypt(&mut self, input: &[u8]) -> Vec<u8>{
        let mut result = [0u8;16];
        let mut state = [[0u8;4];4];

        for i in 0..4 {
            for ii in 0..4 {
                state[i][ii] = input[i * 4 + ii];
            }
        }

        let mut round: usize = 0;

        AdvancedEncryptionStandard::add_round_key(self, &mut state, &mut round);

        round += 1;
        
        for _ in 0..10 {
            AdvancedEncryptionStandard::sub_bytes(self, &mut state);
            AdvancedEncryptionStandard::shift_rows(self, &mut state);
            AdvancedEncryptionStandard::mix_columns(self, &mut state);
            AdvancedEncryptionStandard::add_round_key(self, &mut state, &mut round);
        }

        AdvancedEncryptionStandard::sub_bytes(self, &mut state);
        AdvancedEncryptionStandard::shift_rows(self, &mut state);
        AdvancedEncryptionStandard::add_round_key(self, &mut state, &mut round);

        for i in 0..4 {
            for ii in 0..4 {
                result[ii * 4 + i] = state[i][ii];
            }
        }

        result.to_vec()
    }

    fn decrypt(&mut self, input: &[u8]) -> Vec<u8>{


        input.to_vec()
    }
}


#[cfg(test)]
mod tests {
    use cryptography::AdvancedEncryptionStandard;

    use super::AdvancedEncryptionStandard128Bit;

    #[test]
    fn test_shift_rows() {
        let mut aes = AdvancedEncryptionStandard128Bit { key: [[0u8; 4]; 48] };
        let mut state: [[u8; 4]; 4] = 
        [
            [1, 2, 3, 4],
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 0, 1, 2],
        ];

        /*
        BEFORE        AFTER
        1  2  3  4    1  2  3  4 
        1  2  3  4    2  3  4  1 
        5  6  7  8    7  8  5  6 
        9  0  1  2    2  9  0  1 
        */

        AdvancedEncryptionStandard::shift_rows(&mut aes, &mut state);

        assert_eq!(state[0], [1, 2, 3, 4], "Row 0 did not match expected output.");
        assert_eq!(state[1], [2, 3, 4, 1], "Row 1 did not match expected output.");
        assert_eq!(state[2], [7, 8, 5, 6], "Row 2 did not match expected output.");
        assert_eq!(state[3], [2, 9, 0, 1], "Row 3 did not match expected output.");

    }

    #[test]
    fn test_round_increase_by_add_round_key() {
        let mut aes = AdvancedEncryptionStandard128Bit { key: [[0u8; 4]; 48] };
        let mut state: [[u8; 4]; 4] = 
        [
            [1, 2, 3, 4],
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 0, 1, 2],
        ];

        let mut round: usize = 0;

        for _ in 0..3 {
            AdvancedEncryptionStandard::add_round_key(&mut aes, &mut state, &mut round);
        }

        assert_eq!(round, 3, "Round is wrong!");
    }

    #[test]
    fn test_add_round_key() {
        let mut key: [[u8; 4]; 48] = [[0u8; 4]; 48];
        key[0] = [4, 1, 4, 1];
        key[1] = [3, 2, 3, 2];
        key[2] = [2, 3, 2, 3];
        key[3] = [1, 4, 1, 4];

        let mut aes = AdvancedEncryptionStandard128Bit { key: key };
        let mut state: [[u8; 4]; 4] = 
        [
            [4, 3, 2, 1],
            [1, 2, 3, 4],
            [4, 3, 2, 1],
            [1, 2, 3, 4],
        ];

        let mut round: usize = 0;

        AdvancedEncryptionStandard::add_round_key(&mut aes, &mut state, &mut round);

        assert_eq!(state[0], [0, 0, 0, 0], "Row 0 did not match expected output.");
        assert_eq!(state[1], [0, 0, 0, 0], "Row 1 did not match expected output.");
        assert_eq!(state[2], [0, 0, 0, 0], "Row 2 did not match expected output.");
        assert_eq!(state[3], [0, 0, 0, 0], "Row 3 did not match expected output.");

    }

    #[test]
    fn test_sub_bytes() {
        let mut aes = AdvancedEncryptionStandard128Bit { key: [[0u8; 4]; 48] };
        let mut state: [[u8; 4]; 4] = 
        [
            [0, 3, 69, 180],
            [1, 2, 3, 4],
            [4, 3, 2, 1],
            [1, 2, 3, 4],
        ];

        AdvancedEncryptionStandard::sub_bytes(&mut aes, &mut state);

        assert_eq!(state[0], [0x63, 0x7b, 0x6e, 0x8d], "Values did not match");
    }

    #[test]
    fn test_mix_columns() {
        let mut aes = AdvancedEncryptionStandard128Bit { key: [[0u8; 4]; 48] };
        let mut state: [[u8; 4]; 4] = 
        [
            [0xc6, 0xf2, 0xdb, 0x2d],
            [0xc6, 0x0a, 0x13, 0x26],
            [0xc6, 0x22, 0x53, 0x31],
            [0xc6, 0x5c, 0x45, 0x4c],
        ];

        AdvancedEncryptionStandard::mix_columns(&mut aes, &mut state);

        let col1: Vec<u8> = state.iter().map(|s| *s.iter().nth(0).unwrap()).collect::<Vec<_>>();
        let col2: Vec<u8> = state.iter().map(|s| *s.iter().nth(1).unwrap()).collect::<Vec<_>>();
        let col3: Vec<u8> = state.iter().map(|s| *s.iter().nth(2).unwrap()).collect::<Vec<_>>();
        let col4: Vec<u8> = state.iter().map(|s| *s.iter().nth(3).unwrap()).collect::<Vec<_>>();

        assert_eq!(col1, [0xc6, 0xc6, 0xc6, 0xc6], "Values did not match");
        assert_eq!(col2, [0x9f, 0xdc, 0x58, 0x9d], "Values did not match");
        assert_eq!(col3, [0x8e, 0x4d, 0xa1, 0xbc], "Values did not match");
        assert_eq!(col4, [0x4d, 0x7e, 0xbd, 0xf8], "Values did not match");
    }
}
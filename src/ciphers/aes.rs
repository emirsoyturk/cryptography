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

static RCON: [u8;10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];


pub struct AdvancedEncryptionStandard128Bit {
    round_keys: [[u8; 4]; 44]
}

impl AdvancedEncryptionStandard for AdvancedEncryptionStandard128Bit {
    fn key_schedule(key: [[u8; 4]; 4]) -> [[u8; 4]; 44] {
        let mut round_keys: [[u8; 4]; 44] = [[0; 4]; 44];
    
        round_keys[..4].copy_from_slice(&key);
    
        for i in 4..44 {
            let mut temp = round_keys[i - 1];
    
            if i % 4 == 0 {
                temp = [temp[1], temp[2], temp[3], temp[0]];
    
                for j in 0..4 {
                    temp[j] = SBOX[(temp[j] >> 4) as usize][(temp[j] & 0x0F) as usize];
                }
    
                temp[0] ^= RCON[i / 4 - 1];
            }
    
            for (j, item) in temp.iter().enumerate() {
                round_keys[i][j] = round_keys[i - 4][j] ^ item;
            }
        }
    
        round_keys
    }

    fn add_round_key(&mut self, state: &mut [[u8; 4]; 4], round: &mut usize) {
        let round_key:&[[u8; 4]] = &self.round_keys[*round * 4..(*round + 1) * 4];

        for i in 0..4 {
            for ii in 0..4 {
                state[i][ii] ^= round_key[i][ii];
            }
        }

        *round += 1;
    }

    fn mix_columns(&mut self, state: &mut [[u8; 4]; 4]) {
        let mut temp = [[0u8; 4]; 4];

        for c in 0..4 {
            temp[c][0] = galois_multiplication(0x02, state[c][0]) ^ galois_multiplication(0x03, state[c][1]) ^ state[c][2] ^ state[c][3];
            temp[c][1] = state[c][0] ^ galois_multiplication(0x02, state[c][1]) ^ galois_multiplication(0x03, state[c][2]) ^ state[c][3];
            temp[c][2] = state[c][0] ^ state[c][1] ^ galois_multiplication(0x02, state[c][2]) ^ galois_multiplication(0x03, state[c][3]);
            temp[c][3] = galois_multiplication(0x03, state[c][0]) ^ state[c][1] ^ state[c][2] ^ galois_multiplication(0x02, state[c][3]);
        }

        state.copy_from_slice(&temp);
    }

    fn shift_rows(&mut self, state: &mut [[u8; 4]; 4]) {
        let mut temp = [[0u8; 4]; 4];
        for i in 0..4 {
            for ii in 0..4 {
                temp[i][ii] = state[(i + ii) % 4][ii];
            }
        }

        state.copy_from_slice(&temp);
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
        
        for _ in 0..9 {
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
                result[i * 4 + ii] = state[i][ii];
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
    use cryptography::{AdvancedEncryptionStandard, Cipher};

    use super::AdvancedEncryptionStandard128Bit;

    #[test]
    fn test_shift_rows() {
        // 63 63 7c 7c 
        // 7b 7b c5 c5 
        // 76 76 c0 c0
        // 75 75 d2 d2
        let round_keys = [[0u8; 4]; 44];
        let mut aes = AdvancedEncryptionStandard128Bit { round_keys};
        let mut state: [[u8; 4]; 4] = 
        [
            [0x63, 0x63, 0x7c, 0x7c],
            [0x7b, 0x7b, 0xc5, 0xc5],
            [0x76, 0x76, 0xc0, 0xc0],
            [0x75, 0x75, 0xd2, 0xd2],
        ];

        /*
        BEFORE        AFTER
        63 63 7c 7c   63 7b c0 d2
        7b 7b c5 c5   7b 76 d2 7c
        76 76 c0 c0   76 75 7c c5
        75 75 d2 d2   75 63 c5 c0
        */

        AdvancedEncryptionStandard::shift_rows(&mut aes, &mut state);

        assert_eq!(state[0], [0x63, 0x7b, 0xc0, 0xd2], "Row 0 did not match expected output.");
        assert_eq!(state[1], [0x7b, 0x76, 0xd2, 0x7c], "Row 1 did not match expected output.");
        assert_eq!(state[2], [0x76, 0x75, 0x7c, 0xc5], "Row 2 did not match expected output.");
        assert_eq!(state[3], [0x75, 0x63, 0xc5, 0xc0], "Row 3 did not match expected output.");

    }

    #[test]
    fn test_round_increase_by_add_round_key() {
        let round_keys = [[0u8; 4]; 44];

        let mut aes = AdvancedEncryptionStandard128Bit { round_keys};        
        
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
        let mut round_keys = [[0u8; 4]; 44];

        round_keys[0] = [0x62, 0x63, 0x63, 0x63];
        round_keys[1] = [0x62, 0x63, 0x63, 0x63];
        round_keys[2] = [0x62, 0x63, 0x63, 0x63];
        round_keys[3] = [0x62, 0x63, 0x63, 0x63];

        let mut aes = AdvancedEncryptionStandard128Bit { round_keys };
        let mut state: [[u8; 4]; 4] = 
        [
            [0x59, 0x1c, 0xee, 0xa1],
            [0xc2, 0x86, 0x36, 0xd1],
            [0xca, 0xdd, 0xaf, 0x02],
            [0x4a, 0x27, 0xdc, 0xa2],
        ];

        let mut round: usize = 0;

        AdvancedEncryptionStandard::add_round_key(&mut aes, &mut state, &mut round);

        assert_eq!(state[0], [0x3b, 0x7f, 0x8d, 0xc2], "Values did not match");
        assert_eq!(state[1], [0xa0, 0xe5, 0x55, 0xb2], "Values did not match");
        assert_eq!(state[2], [0xa8, 0xbe, 0xcc, 0x61], "Values did not match");
        assert_eq!(state[3], [0x28, 0x44, 0xbf, 0xc1], "Values did not match");

    }

    #[test]
    fn test_sub_bytes() {
        let round_keys: [[u8; 4]; 44] = [[0u8; 4]; 44];

        let mut aes = AdvancedEncryptionStandard128Bit { round_keys };
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
        let round_keys: [[u8; 4]; 44] = [[0u8; 4]; 44];

        let mut aes = AdvancedEncryptionStandard128Bit { round_keys };
        let mut state: [[u8; 4]; 4] = 
        [
            [0x63, 0x7b, 0xc0, 0xd2],
            [0x7b, 0x76, 0xd2, 0x7c],
            [0x76, 0x75, 0x7c, 0xc5],
            [0x75, 0x63, 0xc5, 0xc0],
        ];

        AdvancedEncryptionStandard::mix_columns(&mut aes, &mut state);

        let col1: Vec<u8> = state.iter().map(|s| *s.iter().nth(0).unwrap()).collect::<Vec<_>>();
        let col2: Vec<u8> = state.iter().map(|s| *s.iter().nth(1).unwrap()).collect::<Vec<_>>();
        let col3: Vec<u8> = state.iter().map(|s| *s.iter().nth(2).unwrap()).collect::<Vec<_>>();
        let col4: Vec<u8> = state.iter().map(|s| *s.iter().nth(3).unwrap()).collect::<Vec<_>>();
        
        assert_eq!(col1, [0x59, 0xc2, 0xca, 0x4a], "Column 1 did not match expected output.");
        assert_eq!(col2, [0x1c, 0x86, 0xdd, 0x27], "Column 2 did not match expected output.");
        assert_eq!(col3, [0xee, 0x36, 0xaf, 0xdc], "Column 3 did not match expected output.");
        assert_eq!(col4, [0xa1, 0xd1, 0x02, 0xa2], "Column 4 did not match expected output.");
    }

    #[test]
    fn test_key_schedule() {
        let base_key: [[u8; 4]; 4] = [
            [0x00, 0x00, 0x00, 0x00],
            [0x00, 0x00, 0x00, 0x00],
            [0x00, 0x00, 0x00, 0x00],
            [0x00, 0x00, 0x00, 0x00]
        ];

        let expected_round_keys: [[[u8; 4]; 4]; 11] = 
        [
            [[0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00]],
            [[0x62, 0x63, 0x63, 0x63], [0x62, 0x63, 0x63, 0x63], [0x62, 0x63, 0x63, 0x63], [0x62, 0x63, 0x63, 0x63]],
            [[0x9b, 0x98, 0x98, 0xc9], [0xf9, 0xfb, 0xfb, 0xaa], [0x9b, 0x98, 0x98, 0xc9], [0xf9, 0xfb, 0xfb, 0xaa]], 
            [[0x90, 0x97, 0x34, 0x50], [0x69, 0x6c, 0xcf, 0xfa], [0xf2, 0xf4, 0x57, 0x33], [0x0b, 0x0f, 0xac, 0x99]],
            [[0xee, 0x06, 0xda, 0x7b], [0x87, 0x6a, 0x15, 0x81], [0x75, 0x9e, 0x42, 0xb2], [0x7e, 0x91, 0xee, 0x2b]],
            [[0x7f, 0x2e, 0x2b, 0x88], [0xf8, 0x44, 0x3e, 0x09], [0x8d, 0xda, 0x7c, 0xbb], [0xf3, 0x4b, 0x92, 0x90]],
            [[0xec, 0x61, 0x4b, 0x85], [0x14, 0x25, 0x75, 0x8c], [0x99, 0xff, 0x09, 0x37], [0x6a, 0xb4, 0x9b, 0xa7]],
            [[0x21, 0x75, 0x17, 0x87], [0x35, 0x50, 0x62, 0x0b], [0xac, 0xaf, 0x6b, 0x3c], [0xc6, 0x1b, 0xf0, 0x9b]],
            [[0x0e, 0xf9, 0x03, 0x33], [0x3b, 0xa9, 0x61, 0x38], [0x97, 0x06, 0x0a, 0x04], [0x51, 0x1d, 0xfa, 0x9f]],
            [[0xb1, 0xd4, 0xd8, 0xe2], [0x8a, 0x7d, 0xb9, 0xda], [0x1d, 0x7b, 0xb3, 0xde], [0x4c, 0x66, 0x49, 0x41]],
            [[0xb4, 0xef, 0x5b, 0xcb], [0x3e, 0x92, 0xe2, 0x11], [0x23, 0xe9, 0x51, 0xcf], [0x6f, 0x8f, 0x18, 0x8e]]
        ];

        let round_keys = AdvancedEncryptionStandard128Bit::key_schedule(base_key);

        for (i, expected_key) in expected_round_keys.iter().enumerate() {
            assert_eq!(round_keys[i*4..(i + 1) * 4], *expected_key, "Round {}: Key schedule does not match expected value", i);
        }

    }

    #[test]
    fn test_encrypt() {
        let base_key: [[u8; 4]; 4] = [[0; 4]; 4];
        let round_keys = AdvancedEncryptionStandard128Bit::key_schedule(base_key);

        let mut aes = AdvancedEncryptionStandard128Bit { round_keys };
        let input:[u8; 16] = [0x00, 0x00, 0x01, 0x01, 0x03, 0x03, 0x07, 0x07, 0x0f, 0x0f, 0x1f, 0x1f, 0x3f, 0x3f, 0x7f, 0x7f];

        let output = Cipher::encrypt(&mut aes, &input);

        let expected_output = [0xc7, 0xd1, 0x24, 0x19, 0x48, 0x9e, 0x3b, 0x62, 0x33, 0xa2, 0xc5, 0xa7, 0xf4, 0x56, 0x31, 0x72];

        assert_eq!(output, expected_output, "Values did not match")
    }

    #[test]
    fn test_encrypt_1_round() {
        let base_key: [[u8; 4]; 4] = [[0; 4]; 4];
        let round_keys = AdvancedEncryptionStandard128Bit::key_schedule(base_key);

        let mut aes = AdvancedEncryptionStandard128Bit { round_keys };
        let input:[u8; 16] = [0x00, 0x00, 0x01, 0x01, 0x03, 0x03, 0x07, 0x07, 0x0f, 0x0f, 0x1f, 0x1f, 0x3f, 0x3f, 0x7f, 0x7f];

        let mut state = [[0u8;4];4];

        for i in 0..4 {
            for ii in 0..4 {
                state[i][ii] = input[i * 4 + ii];
            }
        }

        let mut round: usize = 0;

        AdvancedEncryptionStandard::add_round_key(&mut aes, &mut state, &mut round);
        
        for _ in 0..1 {
            AdvancedEncryptionStandard::sub_bytes(&mut aes, &mut state);
            assert_eq!(state[0], [0x63, 0x63, 0x7c, 0x7c], "SubBytes values did not match");
            assert_eq!(state[1], [0x7b, 0x7b, 0xc5, 0xc5], "SubBytes values did not match");
            assert_eq!(state[2], [0x76, 0x76, 0xc0, 0xc0], "SubBytes values did not match");
            assert_eq!(state[3], [0x75, 0x75, 0xd2, 0xd2], "SubBytes values did not match");

            AdvancedEncryptionStandard::shift_rows(&mut aes, &mut state);
            assert_eq!(state[0], [0x63, 0x7b, 0xc0, 0xd2], "ShiftRows values did not match");
            assert_eq!(state[1], [0x7b, 0x76, 0xd2, 0x7c], "ShiftRows values did not match");
            assert_eq!(state[2], [0x76, 0x75, 0x7c, 0xc5], "ShiftRows values did not match");
            assert_eq!(state[3], [0x75, 0x63, 0xc5, 0xc0], "ShiftRows values did not match");

            AdvancedEncryptionStandard::mix_columns(&mut aes, &mut state);
            assert_eq!(state[0], [0x59, 0x1c, 0xee, 0xa1], "MixColumns values did not match");
            assert_eq!(state[1], [0xc2, 0x86, 0x36, 0xd1], "MixColumns values did not match");
            assert_eq!(state[2], [0xca, 0xdd, 0xaf, 0x02], "MixColumns values did not match");
            assert_eq!(state[3], [0x4a, 0x27, 0xdc, 0xa2], "MixColumns values did not match");

            AdvancedEncryptionStandard::add_round_key(&mut aes, &mut state, &mut round);
            assert_eq!(state[0], [0x3b, 0x7f, 0x8d, 0xc2], "AddRoundKey values did not match");
            assert_eq!(state[1], [0xa0, 0xe5, 0x55, 0xb2], "AddRoundKey values did not match");
            assert_eq!(state[2], [0xa8, 0xbe, 0xcc, 0x61], "AddRoundKey values did not match");
            assert_eq!(state[3], [0x28, 0x44, 0xbf, 0xc1], "AddRoundKey values did not match");
        }
    }
    
    #[test]
    fn test_encrypt_10_round() {
        let base_key: [[u8; 4]; 4] = [[0; 4]; 4];
        let round_keys = AdvancedEncryptionStandard128Bit::key_schedule(base_key);

        let mut aes = AdvancedEncryptionStandard128Bit { round_keys };
        let input:[u8; 16] = [0x00, 0x00, 0x01, 0x01, 0x03, 0x03, 0x07, 0x07, 0x0f, 0x0f, 0x1f, 0x1f, 0x3f, 0x3f, 0x7f, 0x7f];

        let mut state = [[0u8;4];4];

        for i in 0..4 {
            for ii in 0..4 {
                state[i][ii] = input[i * 4 + ii];
            }
        }

        let mut round: usize = 0;

        AdvancedEncryptionStandard::add_round_key(&mut aes, &mut state, &mut round);
        
        for _ in 0..9 {
            AdvancedEncryptionStandard::sub_bytes(&mut aes, &mut state);
            AdvancedEncryptionStandard::shift_rows(&mut aes, &mut state);
            AdvancedEncryptionStandard::mix_columns(&mut aes, &mut state);
            AdvancedEncryptionStandard::add_round_key(&mut aes, &mut state, &mut round);   
        }

        AdvancedEncryptionStandard::sub_bytes(&mut aes, &mut state);
        AdvancedEncryptionStandard::shift_rows(&mut aes, &mut state);
        AdvancedEncryptionStandard::add_round_key(&mut aes, &mut state, &mut round);   

        //c7d12419489e3b6233a2c5a7f4563172

        assert_eq!(state[0], [0xc7, 0xd1, 0x24, 0x19], "Values did not match");
        assert_eq!(state[1], [0x48, 0x9e, 0x3b, 0x62], "Values did not match");
        assert_eq!(state[2], [0x33, 0xa2, 0xc5, 0xa7], "Values did not match");
        assert_eq!(state[3], [0xf4, 0x56, 0x31, 0x72], "Values did not match");

    }




}
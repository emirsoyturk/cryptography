use cryptography::HashFunction;

pub struct MessageDigestAlgorithm {}

impl MessageDigestAlgorithm {
    pub fn new() -> MessageDigestAlgorithm {
        MessageDigestAlgorithm {}
    }

    fn f(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (!x & z)
    }

    fn g(x: u32, y: u32, z: u32) -> u32 {
        (x & z) | (y & !z)
    }

    fn h(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    fn i(x: u32, y: u32, z: u32) -> u32 {
        y ^ (x | !z)
    }

    const S: [[u32; 4]; 4] = [
        [7, 12, 17, 22],
        [5, 9, 14, 20],
        [4, 11, 16, 23],
        [6, 10, 15, 21],
    ];

    const T: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x2441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x4881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];

    const X: [[u32; 16]; 4] = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [1, 6, 11, 0, 5, 10, 15, 4, 9, 14, 3, 8, 13, 2, 7, 12],
        [5, 8, 11, 14, 1, 4, 7, 10, 13, 0, 3, 6, 9, 12, 15, 2],
        [0, 7, 14, 5, 12, 3, 10, 1, 8, 15, 6, 13, 4, 11, 2, 9],
    ];

    fn round1(a: u32, b: u32, c: u32, d: u32, x: u32, i: u32) -> u32 {
        let t = Self::T[i as usize];
        let s = Self::S[0][(i % 4) as usize];
        println!("i: {}, s: {}, t: {:x}", i, s, t);

        a.wrapping_add(Self::f(b, c, d))
            .wrapping_add(x)
            .wrapping_add(t)
            .rotate_left(s)
    }

    fn round2(a: u32, b: u32, c: u32, d: u32, x: u32, i: u32) -> u32 {
        let t = Self::T[i as usize];
        let s = Self::S[1][(i % 4) as usize];
        println!("i: {}, s: {}, t: {:x}", i, s, t);

        a.wrapping_add(Self::g(b, c, d))
            .wrapping_add(x)
            .wrapping_add(t)
            .rotate_left(s)
    }

    fn round3(a: u32, b: u32, c: u32, d: u32, x: u32, i: u32) -> u32 {
        let t = Self::T[i as usize];
        let s = Self::S[2][(i % 4) as usize];
        println!("i: {}, s: {}, t: {:x}", i, s, t);

        a.wrapping_add(Self::h(b, c, d))
            .wrapping_add(x)
            .wrapping_add(t)
            .rotate_left(s)
    }

    fn round4(a: u32, b: u32, c: u32, d: u32, x: u32, i: u32) -> u32 {
        let t = Self::T[i as usize];
        let s = Self::S[3][(i % 4) as usize];
        println!("i: {}, s: {}, t: {:x}", i, s, t);

        a.wrapping_add(Self::i(b, c, d))
            .wrapping_add(x)
            .wrapping_add(t)
            .rotate_left(s)
    }
}

impl Default for MessageDigestAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}

impl HashFunction for MessageDigestAlgorithm {
    //3.1 Step 1. Append Padding Bits
    //3.2 Step 2. Append Length
    //3.3 Step 3. Initialize MD Buffer
    //3.4 Step 4. Process Message in 16-Word Blocks
    //3.5 Step 5. Output
    fn hash(&mut self, input: &[u8]) -> Vec<u8> {
        // Padding
        let mut input_vec: Vec<u8> = input.to_vec();
        let initial_length = input_vec.len() * 8;
        //append "1" bit
        input_vec.push(0x80);
        //append "0" bits until length % 512 == 448
        while input_vec.len() * 8 % 512 != 448 {
            input_vec.push(0x00);
        }
        //append length
        let length = initial_length as u64;
        let length_bytes = length.to_le_bytes();
        (0..8).for_each(|i| input_vec.push(length_bytes[i]));
        // input_vec length should be a multiple of 512 bits
        // initialize MD Buffer
        let (mut a, mut b, mut c, mut d) =
            (0x67452301u32, 0xefcdab89u32, 0x98badcfeu32, 0x10325476u32);
        // Process Message in 16-Word Blocks

        for i in (0..input_vec.len()).step_by(64) {
            let aa = a;
            let bb = b;
            let cc = c;
            let dd = d;
            let mut x: [u32; 16] = [0; 16];

            for j in 0..16 {
                x[j] = u32::from_le_bytes([
                    input_vec[i + j * 4],
                    input_vec[i + j * 4 + 1],
                    input_vec[i + j * 4 + 2],
                    input_vec[i + j * 4 + 3],
                ]);
            }

            let mut step = 0;

            // Round 1
            (0..16).for_each(|j| {
                let x_i = Self::X[0][j] as usize;
                let temp = Self::round1(a, b, c, d, x[x_i], step);
                a = d;
                d = c;
                c = b;
                b = b.wrapping_add(temp);

                step += 1;
            });

            // Round 2
            (0..16).for_each(|j| {
                let x_i = Self::X[1][j] as usize;
                let temp = Self::round2(a, b, c, d, x[x_i], step);
                a = d;
                d = c;
                c = b;
                b = b.wrapping_add(temp);

                step += 1;
            });

            // Round 3
            (0..16).for_each(|j| {
                let x_i = Self::X[2][j] as usize;
                let temp = Self::round3(a, b, c, d, x[x_i], step);
                a = d;
                d = c;
                c = b;
                b = b.wrapping_add(temp);

                step += 1;
            });

            // Round 4
            (0..16).for_each(|j| {
                let x_i = Self::X[3][j] as usize;
                let temp = Self::round4(a, b, c, d, x[x_i], step);
                a = d;
                d = c;
                c = b;
                b = b.wrapping_add(temp);

                step += 1;
            });

            a = a.wrapping_add(aa);
            b = b.wrapping_add(bb);
            c = c.wrapping_add(cc);
            d = d.wrapping_add(dd);
        }

        let mut result: Vec<u8> = Vec::new();
        result.extend_from_slice(&a.to_le_bytes());
        result.extend_from_slice(&b.to_le_bytes());
        result.extend_from_slice(&c.to_le_bytes());
        result.extend_from_slice(&d.to_le_bytes());

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings() {
        let mut md5 = MessageDigestAlgorithm {};
        let input = "";
        let output = md5.hash(input.as_bytes());
        assert_eq!(
            output,
            vec![
                0xd4, 0x1d, 0x8c, 0xd9, 0x8f, 0x00, 0xb2, 0x04, 0xe9, 0x80, 0x09, 0x98, 0xec, 0xf8,
                0x42, 0x7e
            ]
        );

        let input = "a";
        let output = md5.hash(input.as_bytes());
        assert_eq!(
            output,
            vec![
                0x0c, 0xc1, 0x75, 0xb9, 0xc0, 0xf1, 0xb6, 0xa8, 0x31, 0xc3, 0x99, 0xe2, 0x69, 0x77,
                0x26, 0x61
            ]
        );

        let input = "abc";
        let output = md5.hash(input.as_bytes());
        assert_eq!(
            output,
            vec![
                0x90, 0x01, 0x50, 0x98, 0x3c, 0xd2, 0x4f, 0xb0, 0xd6, 0x96, 0x3f, 0x7d, 0x28, 0xe1,
                0x7f, 0x72
            ]
        );

        let input = "message digest";
        let output = md5.hash(input.as_bytes());
        assert_eq!(
            output,
            vec![
                0xf9, 0x6b, 0x69, 0x7d, 0x7c, 0xb7, 0x93, 0x8d, 0x52, 0x5a, 0x2f, 0x31, 0xaa, 0xf1,
                0x61, 0xd0
            ]
        );

        let input = "abcdefghijklmnopqrstuvwxyz";
        let output = md5.hash(input.as_bytes());
        assert_eq!(
            output,
            vec![
                0xc3, 0xfc, 0xd3, 0xd7, 0x61, 0x92, 0xe4, 0x00, 0x7d, 0xfb, 0x49, 0x6c, 0xca, 0x67,
                0xe1, 0x3b
            ]
        );

        let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let output = md5.hash(input.as_bytes());
        assert_eq!(
            output,
            vec![
                0xd1, 0x74, 0xab, 0x98, 0xd2, 0x77, 0xd9, 0xf5, 0xa5, 0x61, 0x1c, 0x2c, 0x9f, 0x41,
                0x9d, 0x9f
            ]
        );

        let input =
            "12345678901234567890123456789012345678901234567890123456789012345678901234567890";
        let output = md5.hash(input.as_bytes());
        assert_eq!(
            output,
            vec![
                0x57, 0xed, 0xf4, 0xa2, 0x2b, 0xe3, 0xc9, 0x55, 0xac, 0x49, 0xda, 0x2e, 0x21, 0x07,
                0xb6, 0x7a
            ]
        );
    }
}

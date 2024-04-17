pub const ID_LENGTH: usize = 20;

pub struct UID {
    pub(crate) bid: [u8; ID_LENGTH],
}

impl From<[u8; ID_LENGTH]> for UID {

    fn from(bid: [u8; ID_LENGTH]) -> Self {
        Self {
            bid
        }
    }
}

impl From<&str> for UID {

    fn from(key: &str) -> Self {
        if key.len() != ID_LENGTH * 2 {
            panic!("Node ID is not correct length");
            //return Err("Node ID is not correct length");
        }

        let mut bid = [0u8; ID_LENGTH];
        for (i, chunk) in key.as_bytes().chunks(2).enumerate() {
            let byte = match u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16) {
                Ok(byte) => byte,
                Err(_) => panic!("Invalid hex string"),
            };
            bid[i] = byte;
        }

        Self {
            bid
        }
    }
}

impl UID {

    pub fn distance(&self, k: &UID) -> usize {
        ID_LENGTH - self.xor(k).first_set_bit_index()
    }

    fn xor(&self, k: &UID) -> UID {
        let mut distance = [0u8; ID_LENGTH];
        for i in 0..ID_LENGTH {
            distance[i] = self.bid[i] ^ k.bid[i];
        }
        UID { bid: distance }
    }


    fn first_set_bit_index(&self) -> usize {
        let mut prefix_length = 0;
        for &b in &self.bid {
            if b == 0 {
                prefix_length += 8;
            } else {
                let count = b.leading_zeros() as usize;
                prefix_length += count;
                break;
            }
        }
        prefix_length
    }

    //PRODUCING INCORRECT NUMBERS...
    /*
    pub fn generate_node_id_by_distance(&self, distance: usize) -> UID {
        let mut result = [0u8; ID_LENGTH];

        let num_byte_zeroes = ((ID_LENGTH * 8) - distance) / 8;
        let num_bit_zeroes = 8 - (distance % 8);

        for i in 0..num_byte_zeroes {
            result[i] = 0;
        }

        let mut bits = BitVec::from_elem(8, true);
        for i in 0..num_bit_zeroes {
            bits.set(i, false);
        }

        //bits.flip();
        let byte = bits.to_bytes()[0];
        result[num_byte_zeroes] = byte;

        for i in num_byte_zeroes + 1..result.len() {
            result[i] = u8::MAX;
        }

        self.xor(&UID { bid: result })
    }
    */
    pub fn generate_node_id_by_distance(&self, distance: usize) -> UID {
        let mut result = [0u8; ID_LENGTH];

        let num_byte_zeroes = ((ID_LENGTH * 8) - distance) / 8;
        let num_bit_zeroes = 8 - (distance % 8);

        // Set the first `num_byte_zeroes` bytes to 0
        for i in 0..num_byte_zeroes {
            result[i] = 0;
        }

        // Set the bits in the next byte based on `num_bit_zeroes`
        let mut byte = 0u8;
        for i in 0..num_bit_zeroes {
            byte |= 1 << (7 - i); // Set the bit at position 7 - i to 1
        }
        result[num_byte_zeroes] = byte;

        // Set the remaining bytes to u8::MAX
        for i in num_byte_zeroes + 1..ID_LENGTH {
            result[i] = u8::MAX;
        }

        self.xor(&UID { bid: result })
    }

    pub fn bytes(&self) -> [u8; ID_LENGTH] {
        self.bid
    }

    pub fn binary(&self) -> String {
        let mut binary = String::new();
        for &b in &self.bid {
            binary.push_str(&format!("{:08b}", b));
        }
        binary
    }

    pub fn hex(&self) -> String {
        let mut hex_string = String::with_capacity(ID_LENGTH * 2);

        for &byte in self.bid.iter().take(ID_LENGTH) {
            hex_string.push_str(&format!("{:02x}", byte));
        }

        hex_string
    }

    pub fn to_string(&self) -> String {
        let mut hex_string = String::with_capacity(ID_LENGTH * 2);

        // Append the first 3 bytes with leading zeros if needed
        for &byte in self.bid.iter().take(3) {
            hex_string.push_str(&format!("{:02x}", byte));
        }
        hex_string.push(' ');

        // Append bytes from index 3 to 18
        for &byte in self.bid.iter().take(19).skip(3) {
            hex_string.push_str(&format!("{:02x}", byte));
        }

        // Append the last byte with leading zero if needed
        hex_string.push(' ');
        hex_string.push_str(&format!("{:02x}", self.bid[19]));

        hex_string
    }
}

impl PartialEq for UID {
    fn eq(&self, other: &Self) -> bool {
        self.bid == other.bid
    }
}

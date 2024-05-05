pub const ID_LENGTH: usize = 20;

#[derive(Debug, Copy, Clone)]
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
/*
impl From<Vec<u8>> for UID {

    fn from(bid: Vec<u8>) -> Self {
        let bid: [u8; ID_LENGTH] = match bid.as_slice() {
            [x @ _, rest @ ..] if rest.len() == ID_LENGTH - 1 => {
                let mut array = [0; ID_LENGTH];
                array[0] = *x;
                array.copy_from_slice(&rest);
                array
            }
            _ => panic!("Vector size is not equal to array size"),
        };

        Self {
            bid
        }
    }
}
*/
impl TryFrom<&str> for UID {

    type Error = String;

    fn try_from(key: &str) -> Result<Self, Self::Error> {
        if key.len() != ID_LENGTH * 2 {
            return Err(format!("Node ID is not correct length, given string is {} chars, required {} chars", key.len(), ID_LENGTH));
        }

        let mut bid = [0u8; ID_LENGTH];
        for (i, chunk) in key.as_bytes().chunks(2).enumerate() {
            let byte = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).map_err(|e| e.to_string())?;
            bid[i] = byte;
        }

        Ok(Self {
            bid
        })
    }
}

impl UID {

    pub fn distance(&self, k: &UID) -> usize {
        ID_LENGTH*8-self.xor(k).first_set_bit_index()
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

    pub fn generate_node_id_by_distance(&self, distance: usize) -> UID {
        let mut result = [0; ID_LENGTH];
        let num_byte_zeroes = ((ID_LENGTH * 8) - distance) / 8;
        let num_bit_zeroes = (8 - distance % 8) % 8;

        for i in 0..num_byte_zeroes {
            result[i] = 0;
        }

        let mut bits = [true; 8];
        for i in 0..num_bit_zeroes {
            bits[i] = false;
        }

        for i in 0..8 {
            if bits[i] {
                result[num_byte_zeroes] |= 1 << (7 - i);
            }
        }

        for i in (num_byte_zeroes + 1)..ID_LENGTH {
            result[i] = std::u8::MAX;
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

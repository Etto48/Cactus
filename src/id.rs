use std::{fmt::Display, hash::{Hash, Hasher}, ops::{Index, IndexMut}, str::FromStr};

const ID_SIZE: usize = 8;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Id {
    id: [u8; ID_SIZE]
}

impl Id {
    pub fn zero() -> Self {
        Self { id: [0; ID_SIZE] }
    }

    pub fn new(id: [u8; ID_SIZE]) -> Self {
        Self { id }
    }

    pub fn from_key<T: Hash>(key: T) -> Self {
        let mut hasher = std::hash::DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish().to_be_bytes();
        Self::new(hash)
    }

    /// compute the difference between two IDs modulo 2^(8*ID_SIZE)
    pub fn distance(&self, other: &Self) -> Self {
        let mut distance = Self::zero();
        let mut carry = 0;
        for i in (0..ID_SIZE).rev() {
            let (result, new_carry) = self.id[i].overflowing_sub(other.id[i] + carry);
            distance[i] = result;
            carry = new_carry as u8;
        }
        distance
    }

    /// Get the i-th digith of the ID, a digit is 4bits
    pub fn get_digit(&self, i: usize) -> u8 {
        let byte = self.id[i / 2];
        if i % 2 == 0 {
            byte & 0x0F
        } else {
            byte >> 4
        }
    }
}

impl FromStr for Id {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = Self::zero();
        let mut i = 0;
        for c in s.chars() {
            if c=='-' {
                continue;
            }
            if (i % 2) == 0 {
                id[i / 2] = match c.to_digit(16) {
                    Some(d) => d as u8,
                    None => return Err(format!("Invalid character: {}, expected hex digit", c))
                };
            } else {
                id[i / 2] |= match c.to_digit(16) {
                    Some(d) => (d as u8) << 4,
                    None => return Err(format!("Invalid character: {}, expected hex digit", c))
                };
            }
            i += 1;
        }
        if i != ID_SIZE * 2 {
            return Err(format!("Invalid ID length ('-' excluded): {}, expected {}", s.len(), ID_SIZE * 2));
        }
        Ok(id)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.id.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl Index<usize> for Id {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.id[index / 2]
    }
}

impl IndexMut<usize> for Id {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.id[index]
    }
}

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in 0..ID_SIZE {
            match self.id[i].cmp(&other.id[i]) {
                std::cmp::Ordering::Equal => continue,
                ord => return ord
            }
        }
        std::cmp::Ordering::Equal
    }
}
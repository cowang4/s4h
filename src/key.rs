
use std::cmp::Ordering;


pub const KEY_SIZE_BITS: usize = 128; // in bits
pub const KEY_SIZE_BYTES: usize = KEY_SIZE_BITS / 8;

// A big endian stored key
pub type Key = [u8; KEY_SIZE_BYTES];


/// Compares two keys, returns an std::cmp::Ordering
pub fn key_cmp(k1: &Key, k2: &Key) -> Ordering {
    for (b1, b2) in k1.iter().zip(k2.iter()) {
        if b1 > b2 {
            return Ordering::Greater;
        }
        else if b1 < b2 {
            return Ordering::Less;
        }
        // else continue
    }
    Ordering::Equal
}

pub fn key_dist(k1: &Key, k2: &Key) -> Key {
    let mut dist: Key = [0; KEY_SIZE_BYTES];
    for (i, (b1, b2)) in k1.iter().zip(k2.iter()).enumerate() {
        dist[i] = b1 ^ b2;
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_key_cmp() {
        let x1 = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let x2 = [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let x3 = [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(key_cmp(&x1, &x2), Ordering::Greater);
        assert_eq!(key_cmp(&x2, &x1), Ordering::Less);
        assert_eq!(key_cmp(&x1, &x1), Ordering::Equal);
        assert_eq!(key_cmp(&x2, &x2), Ordering::Equal);
        assert_eq!(key_cmp(&x3, &x3), Ordering::Equal);
        assert_eq!(key_cmp(&x1, &x3), Ordering::Less);
        assert_eq!(key_cmp(&x2, &x3), Ordering::Less);
    }

    #[test]
    fn test_key_dist() {
        let x1 = [255; 16];
        let x2 = [0; 16];
        let x3 = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(key_dist(&x1, &x2), [255; 16]);
        assert_eq!(key_dist(&x1, &x1), [0; 16]);
        assert_eq!(key_dist(&x2, &x2), [0; 16]);
        assert_eq!(key_dist(&x2, &x3), [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(key_dist(&x1, &x3), [254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
    }
}

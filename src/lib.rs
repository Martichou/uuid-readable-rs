//! TODO DOCUMENT THE LIB FOR CRATES.IO

use data::*;
use uuid::Uuid;

use crate::data::{
    adjectives::ADJECTIVES, animals::ANIMALS, personal_nouns::PERSONAL_NOUNS, places::PLACES,
    verbs::VERBS,
};

mod data;

/// Mask used for the long version, this allow us to convert a 16 items
/// totalling 128 bits into 12 items for the same number of bits.
const NORMAL: [u8; 12] = [12, 11, 14, 13, 13, 10, 12, 11, 14, 5, 6, 7];
/// Used for low entropy in the short methods. Higher chances of collisions
/// between two generated sentences. 128 bits into 5 items.
const SHORT: [u8; 5] = [17, 25, 30, 29, 27];

/// Convert an array of bytes to a Vec to individuals bits (1-0)
fn to_bits(bytes: &[u8]) -> Vec<u8> {
    let mut bits: Vec<u8> = Vec::with_capacity(128);

    for b in bytes {
        let mut b = *b;
        for _ in 0..8 {
            bits.push(b % 2);
            b >>= 1;
        }
    }

    bits
}

/// Convert an array of individuals bits to a byte
fn to_byte(bits: &[u8]) -> u16 {
    let mut _byte = 0u16;
    for b in bits {
        _byte = 2 * (_byte % 32766) + *b as u16;
    }
    _byte
}

/// Convert bytes to bits and group them into 12 distinct numbers
fn partition(parts: &[u8], bytes: &[u8]) -> [usize; 12] {
    let mut bits: Vec<u8> = to_bits(bytes);

    let mut _bytes: [usize; 12] = [0; 12];
    for (idx, p) in parts.iter().enumerate() {
        let tmp = bits.drain(0..(*p as usize));
        _bytes[idx] = to_byte(tmp.as_slice()) as usize;
    }

    _bytes
}

#[inline]
/// TODO - Document
fn _generate(uuid: &Uuid) -> String {
    // Convert the Uuid to an array of bytes
    let uuid = uuid.as_bytes();
    // Get the partition (it's basically random numbers (12) from the uuid)
    let words = partition(&NORMAL, uuid);

    // Generate the sentence and return it
    format!(
        "{} {} {} the {} of {} {} {} {} {} and {} {} {}",
        names::FIRST[words[0] % 25],
        names::MIDDLE[words[1] % 25],
        names::LAST[words[2] % 25],
        PERSONAL_NOUNS[words[3] % 25],
        PLACES[words[4] % 25],
        VERBS[words[5] % 25],
        names::FIRST[words[6] % 25],
        names::MIDDLE[words[7] % 25],
        names::LAST[words[8] % 25],
        words[9] % 32,
        ADJECTIVES[words[10] % 25],
        ANIMALS[words[11] % 25]
    )
}

/// TODO - Document
pub fn generate() -> String {
    // Generate a new Uuid using the v4 RFC
    let uuid = Uuid::new_v4();
    // Create the sentence from the Uuid
    _generate(&uuid)
}

/// TODO - Document
pub fn generate_from(uuid: Uuid) -> String {
    // Create the sentence from the Uuid
    _generate(&uuid)
}

#[inline]
/// TODO - Document
fn _short(uuid: &Uuid) -> String {
    // Convert the Uuid to an array of bytes
    let uuid = uuid.as_bytes();
    // Get the partition (it's basically random numbers (12) from the uuid)
    let words = partition(&SHORT, uuid);

    // Generate the sentence and return it
    format!(
        "{} {} by {} {} {}",
        names::FIRST[words[0] % 25],
        VERBS[words[1] % 25],
        words[2] % 64,
        ADJECTIVES[words[3] % 25],
        ANIMALS[words[4] % 25],
    )
}

/// TODO - Document
pub fn short() -> String {
    // Generate a new Uuid using the v4 RFC
    let uuid = Uuid::new_v4();
    // Create the sentence from the Uuid
    _short(&uuid)
}

/// TODO - Document
pub fn short_from(uuid: Uuid) -> String {
    // Create the sentence from the Uuid
    _short(&uuid)
}

#[test]
fn sanity() {
    let uuid = Uuid::parse_str("0ee001c7-12f3-4b29-a4cc-f48838b3587a").unwrap();

    let g = generate_from(uuid);
    assert_eq!(
        g,
        "Joy Bolt Kahler the avenger of Esbon jumped Carey Fatma Sander and 8 large ducks"
    );

    let s = short_from(uuid);
    assert_eq!(s, "Cookie snuggled by 45 far weasels");
}

//! Generate easy to remember sentences that acts as human readable UUIDs.
//!
//! - Built on UUID v4
//! - Optionally pass your UUID to derive a sentence from it
//! - Grammatically _correct_ sentences
//! - Easy to remember (or at least part of it)
//! - Default (`generate()`) to 128 bits Token using the [UUID](https://docs.rs/uuid) crates.
//! - Generate low entropy (`short()`) 32 bits Token
//!
//! ## Security
//! This project does not mean to be crypto safe ! **Don't use this as a secure random generator**.
//!
//! Even if we derive sentences from UUID (that are crypto safe), there can still be some collision
//! with 2 differents UUID but resulting in the same sentence.
//!
//! - `25^12` possible combinations for `generate()` (uses 128 bits Token)
//! - `25^5` possible combinations for `short()` (uses 32 bits Token)
//!
//! ## Example
//! ```
//! use uuid::Uuid;
//! use uuid_readable_rs::{generate_from, short_from, generate, short};
//!
//! // You can define your own UUID and pass it to uuid_readable_rs like so
//! let uuid = Uuid::new_v4();
//! let sentence_128: String = generate_from(uuid);
//! let sentence_32: String = short_from(uuid);
//!
//! // Or let uuid_readable_rs handle the Uuid generation
//! let sentence_128: String = generate();
//! let sentence_32: String = short();
//! ```

use data::*;
use uuid::Uuid;

use crate::data::{
    adjectives::ADJECTIVES, animals::ANIMALS, personal_nouns::PERSONAL_NOUNS, places::PLACES,
    verbs::VERBS,
};

mod data;

// TODO - To avoid losing entropy, we should have lists of:
// - 2^12 elements for NORMAL[0]
// - 2^11 elements for NORMAL[1]
// - 2^14 elements for NORMAL[2]
// - ...
//
// TODO - Add a reverse method for sentence -> uuid

/// Mask used for the long version, this allow us to convert a 16 items
/// totalling 128 bits into 12 items for the same number of bits.
const NORMAL: [u8; 12] = [12, 11, 14, 13, 13, 10, 12, 11, 14, 5, 6, 7];
/// Used for low entropy in the short methods. Higher chances of collisions
/// between two generated sentences. 32 bits into 5 items.
const SHORT: [u8; 5] = [6, 6, 7, 8, 5];

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
        _byte = 2 * _byte + *b as u16;
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

/// Create a long sentence using a new random uuid.
///
/// Example of return: `Joy Bolt Kahler the avenger of Esbon jumped Carey Fatma Sander and 8 large ducks`
pub fn generate() -> String {
    // Generate a new Uuid using the v4 RFC
    let uuid = Uuid::new_v4();
    // Create the sentence from the Uuid
    _generate(&uuid)
}

/// Derive a long sentence from a uuid.
///
/// Example of return: `Joy Bolt Kahler the avenger of Esbon jumped Carey Fatma Sander and 8 large ducks`
pub fn generate_from(uuid: Uuid) -> String {
    // Create the sentence from the Uuid
    _generate(&uuid)
}

#[inline]
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

/// Create a short sentence using a new random uuid.
///
/// Example of return: `Alex sang by 60 narrow chickens`
pub fn short() -> String {
    // Generate a new Uuid using the v4 RFC
    let uuid = Uuid::new_v4();
    // Create the sentence from the Uuid
    _short(&uuid)
}

/// Derive a short sentence from a uuid.
///
/// Example of return: `Alex sang by 60 narrow chickens`
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
    assert_eq!(s, "Alex sang by 60 narrow chickens");
}

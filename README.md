<div align="center">
  <h1>uuid-readable-rs</h1>
  <p>
    <strong>Easy to remember unique sentences acting as UUID</strong>
  </p>
  <p>

[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/uuid-readable-rs.svg)](https://crates.io/crates/uuid-readable-rs)
[![Released API docs](https://docs.rs/uuid-readable-rs/badge.svg)](https://docs.rs/uuid-readable-rs)
[![CI](https://github.com/Martichou/uuid-readable-rs/workflows/CI/badge.svg)](https://github.com/Martichou/uuid-readable-rs/actions)

  </p>
</div>

Generate easy to remember sentences that acts as human readable UUIDs.

- Built on UUID v4
- Optionally pass your UUID to derive a sentence from it
- Grammatically _correct_ sentences
- Easy to remember (or at least part of it)
- Size choice (32-bit token or 128-bit token using `short()` or `generate()` respectively)

## Status
Even if this project doesn't receive a lot of update, it's not abandonned.

There's just not much to do as any changes would pretty much break the entire compatibility.
But if any security issue arise, I'll take care of it asap.

## Security
This project does not mean to be crypto safe ! **Don't use this as a secure random generator**.

- `25^12` possible combinations for `generate()` (uses 128-bit Token)
- `25^5` possible combinations for `short()` (uses 32-bit Token)

Note that the sentence generated by `generate()` and the original UUID form a bijection, hence no loss of entropy.

## Sentence generated
For the **long** - aka `generate()` - version, a typical sentence generated by this lib looks like:
```
Wildon Mollie Behka the bubbler of Arecibo moaned Chavey Haney Torbart and 10 calm kingfishers
```
Internally this correspond to:
- 12 bits for a name
- 11 bits for a name
- 14 bits for a name
- 13 bits for a personal noun
- 13 bits for a place
- 10 bits for a verb
- 12 bits for a name
- 11 bits for a name
- 14 bits for a name
- 5 bits for a number
- 6 bits for an adjective
- 7 bits for an animal

> To ensure no loss of entropy, taking the example of the verb which represents 10 bits, this means that we used a list of verbs of at least 2^10 possibilities (1024).

For the **short** - aka `short()` - version, a typical sentence looks like:
```
Zink recorded by 127 large armadillos
```
This correspond to:
- 6 bits for a name
- 6 bits for a verb
- 7 bits for a number
- 8 bits for an adjective
- 5 bits for an animal

> Since the short version is 32 bits long and is derived from a 128-bit UUID, it is not considered as secure or as random as the long version may be. It also does not form any bijection with the original UUID.

## Example
```rust
use uuid::Uuid;
use uuid_readable_rs::{generate_from, short_from, generate, short, generate_inverse};

// You can define your own UUID and pass it to uuid_readable_rs like so
let uuid = Uuid::new_v4();
let sentence_128: String = generate_from(uuid);
let sentence_32: String = short_from(uuid);

// You can also get an UUID from a sentence that was previously generated
let original_uuid: Uuid = generate_inverse(sentence_128).unwrap();
assert_eq!(uuid, original_uuid);

// Or let uuid_readable_rs handle the Uuid generation
let sentence_128: String = generate();
let sentence_32: String = short();
```

## Credits

Thanks to @Debdut for the original idea (https://github.com/Debdut/uuid-readable).

_<sub>note: his version may not be suitable to use at the moment until https://github.com/Debdut/uuid-readable/issues/8 get fixed.</sub>_

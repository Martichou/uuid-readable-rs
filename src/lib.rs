use uuid::Uuid;

// List of animals and adjectives from https://github.com/Debdut/uuid-readable

static ANIMALS: [&str; 25] = [
    "rabbits",
    "badgers",
    "foxes",
    "chickens",
    "bats",
    "deer",
    "snakes",
    "hares",
    "hedgehogs",
    "platypuses",
    "moles",
    "mice",
    "otters",
    "rats",
    "squirrels",
    "stoats",
    "weasels",
    "crows",
    "doves",
    "ducks",
    "geese",
    "hawks",
    "herons",
    "kingfishers",
    "owls",
];

static ADJECTIVES: [&str; 25] = [
    "cute", "dapper", "large", "small", "long", "short", "thick", "narrow", "deep", "flat",
    "whole", "low", "high", "near", "far", "fast", "quick", "slow", "early", "late", "bright",
    "dark", "cloudy", "warm", "cool",
];

pub fn generate() -> String {
    let uuid = Uuid::new_v4();
    let uuid = uuid.as_bytes();

    let mut ret = String::new();

    for i in (0..15).step_by(8) {
        match i {
            0 => {
                ret.push_str(ADJECTIVES[(uuid[i] & uuid[i + 1]) as usize % ADJECTIVES.len()]);
                ret.push('_');
            }
            _ => ret.push_str(ANIMALS[(uuid[i] & uuid[i + 1]) as usize % ANIMALS.len()]),
        }
    }
    ret
}

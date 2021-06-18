const HANDSHAKE_MAGIC_NUMBER: u64 = 20201227;

pub fn derive_loop_size(subject_number: u64, target_number: u64) -> u64 {
    let mut value = 1;
    let mut loop_size = 0;

    loop {
        loop_size += 1;

        value = handshake_step(value, subject_number);

        if value == target_number {
            return loop_size;
        }
    }
}

pub fn derive_encryption_key(public_key: u64, loop_size: u64) -> u64 {
    let mut value = 1;

    for _ in 0..loop_size {
        value = handshake_step(value, public_key);
    }

    value
}

fn handshake_step(value: u64, subject_number: u64) -> u64 {
    (value * subject_number) % HANDSHAKE_MAGIC_NUMBER
}
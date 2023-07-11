pub type Bit = bool;

pub enum HammingDecodeResult {
    NoError {
        decoded_bits: Vec<Bit>,
    },
    OneError {
        position: usize, // Where the error occurred
        decoded_bits: Vec<Bit>,
    },
    DoubleError,
}

pub fn encode(bits: &Vec<Bit>) -> Vec<Bit> {
    let mut extra_bits = 0;
    let data_len = bits.len();

    // Count extra bits for encoding
    while (data_len + extra_bits + 1) > (1 << extra_bits) {
        extra_bits += 1;
    }

    let mut encoded_data = vec![false]; // Add one bit for the whole block parity
    let mut whole_block_partity = false;
    let mut current_parity_bit_number = 0;

    // Create encoded vector
    for i in 0..data_len + extra_bits {
        let current_parity_bit = 1 << current_parity_bit_number;

        if current_parity_bit == i + 1 {
            // parity bits
            encoded_data.push(false);
            current_parity_bit_number += 1;
        } else {
            // data bits
            let new_bit = bits[i - current_parity_bit_number];
            encoded_data.push(new_bit);
            whole_block_partity ^= new_bit;
        }
    }

    // Count parity bits
    for i in 0..extra_bits {
        let mask = 1 << i;
        let mut parity_bit = false;

        for j in 1..encoded_data.len() {
            if j & mask == mask {
                parity_bit ^= encoded_data[j];
            }
        }

        encoded_data[mask] = parity_bit;
        whole_block_partity ^= parity_bit;
    }

    // Set bit for the whole block parity
    encoded_data[0] = whole_block_partity;
    encoded_data
}

pub fn decode(bits: &mut Vec<Bit>) -> HammingDecodeResult {
    // Count parity_bits_number
    let mut parity_bits_number = 0;
    while 1 << parity_bits_number < bits.len() {
        parity_bits_number += 1;
    }

    let block_parity = bits.iter().fold(false, |sum, value| sum ^ *value);
    let mut wrong_parity_bits = Vec::new();

    for i in 0..parity_bits_number {
        let mask = 1 << i;
        let mut parity_bits_sum = 0;

        for j in 1..bits.len() {
            if j & mask == mask && bits[j] {
                parity_bits_sum += 1;
            }
        }

        if parity_bits_sum % 2 != 0 {
            wrong_parity_bits.push(mask);
        }
    }

    let decoded_bits = bits.len() - parity_bits_number;

    if !block_parity && wrong_parity_bits.len() == 0 {
        let decoded_bits = get_inner_data(&bits, decoded_bits);
        return HammingDecodeResult::NoError { decoded_bits };
    } else if block_parity {
        // One mistake
        let position = wrong_parity_bits.iter().sum();
        let opposing = if bits[position] { false } else { true };
        bits[position] = opposing;
        let decoded_bits = get_inner_data(&bits, decoded_bits);
        return HammingDecodeResult::OneError {
            position,
            decoded_bits,
        };
    } else {
        // Two mistakes
        return HammingDecodeResult::DoubleError;
    }
}

fn get_inner_data(encoded_bits: &Vec<Bit>, decoded_size: usize) -> Vec<Bit> {
    let mut inner_data = Vec::with_capacity(decoded_size);
    let mut power_counter = 0;

    for i in 1..encoded_bits.len() {
        if 1 << power_counter == i {
            power_counter += 1;
            continue;
        } else {
            inner_data.push(encoded_bits[i]);
        }
    }

    inner_data
}

pub fn bit_vector_from_bytes(bytes: &Vec<u8>) -> Vec<Bit> {
    let mut bits = vec![];
    for byte in bytes {
        let mut tmp = *byte;
        let mut byte_vector = Vec::with_capacity(8);
        while tmp != 0 {
            if tmp % 2 == 0 {
                byte_vector.push(false);
            } else {
                byte_vector.push(true);
            }
            tmp >>= 1;
        }

        while byte_vector.len() % 8 != 0 {
            byte_vector.push(false)
        }

        byte_vector.reverse();
        bits.append(&mut byte_vector);
    }

    bits
}

pub fn bit_vector_to_bytes(bits: &Vec<Bit>) -> Vec<u8> {
    let bytes_number = bits.len() / 8 + if bits.len() % 8 > 0 { 1 } else { 0 };
    let mut byte_vector = vec![0; bytes_number];
    let mut byte = 0;
    for i in 0..bits.len() {
        if bits[i] {
            byte += 1 << 7 - (i % 8);
        }

        if (i + 1) % 8 == 0 {
            byte_vector[i / 8] = byte;
            byte = 0;
        }
    }

    byte_vector
}

pub fn bit_vector_to_string(bits: &Vec<Bit>) -> String {
    let mut str = "".to_owned();
    for i in 0..bits.len() {
        if bits[i] {
            str.push_str("1");
        } else {
            str.push_str("0");
        }
    }
    str
}


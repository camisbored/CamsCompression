use std::io::{Write, Error};
use super::pattern::Pattern;

/// print_help
/// prints out information about program and usage.
pub fn print_help() {
    println!("Cams Compression, @Grande Software Solutions, 2023");
    println!("This is a custom compression algorithm focusing on repeated data/padding.");
    println!("Usage: cams_compression [-c|-d] <input_file> <output_file>");
    println!("Options:");
    println!("  -c    Compress the input file");
    println!("  -d    Decompress the input file");
}

/// get_byte_count_from_int
/// takes a integer and returns how many bytes 
/// it would take to describe that integer
pub fn get_byte_count_from_int(address: u32) -> u8 {
    match address {
        0..=255 => 1,   // 8-bit integer range
        256..=65535 => 2,  // 16-bit integer range
        65536..=16777215 => 3,  // 24-bit integer range
        _ => 4,  // 32-bit integer range (default case)
    }
}

/// get_int_from_vec
/// takes a unsigned char describing how many bytes to read,
/// and an unsigned char array. it will read the number of bytes
/// from the array and turn it into the descired [1-4 byte] integer 
pub fn get_int_from_vec(value: u8, buffer: &[u8]) -> usize {
    let num_bytes = value as usize;

    let bytes = &buffer[..num_bytes];
    let mut result = 0;

    for &byte in bytes {
        result = (result << 8) | byte as usize;
    }

    result
}

/// get_byte_series_from_int
/// will take a integer, and will turn it into a series of bytes to describe
/// returned as a u8 vec
pub fn get_byte_series_from_int(value: u32) -> Result<Vec<u8>, Error> {
    let mut local_data: Vec<u8> = Vec::new();
    if value <= 0xFF {
        local_data.write_all(&[value as u8])?;
    } else if value <= 0xFFFF {
        local_data.write_all(&[(value >> 8) as u8, value as u8])?;
    } else if value <= 0xFFFFFF {
        local_data.write_all(&[(value >> 16) as u8, (value >> 8) as u8, value as u8])?;
    } else {
        local_data.write_all(&[(value >> 24) as u8, (value >> 16) as u8, (value >> 8) as u8, value as u8])?;
    }
    return Ok(local_data);
}

/// parse patterns
/// parses pattern table and returns vec with completed structs
pub fn parse_patterns(pattern_data: &[u8]) -> Vec<Pattern> {
    let mut patterns: Vec<Pattern> = Vec::new();
    let mut i = 0;

    while i < pattern_data.len() {
        //read each value from memory
        let bytes_to_desc_addr: u8 = pattern_data[i];
        i += 1;

        let pattern_addr_bytes = pattern_data[i..(bytes_to_desc_addr as usize + i) as usize].to_vec();
        i += bytes_to_desc_addr as usize;

        let pattern_addr = get_int_from_vec(bytes_to_desc_addr, &pattern_addr_bytes);

        let bytes_to_desc_count: u8 = pattern_data[i];
        i += 1;

        let pattern_count_bytes = pattern_data[i..(bytes_to_desc_count as usize + i) as usize].to_vec();
        i += bytes_to_desc_count as usize;

        let pattern_count = get_int_from_vec(bytes_to_desc_count, &pattern_count_bytes);

        let character = pattern_data[i];
        i += 1;

        //put into struct
        let pattern = Pattern::new_deserialize(
            bytes_to_desc_addr,
            pattern_addr.try_into().unwrap(),
            bytes_to_desc_count,
            pattern_count.try_into().unwrap(),
            character,
        );

        patterns.push(pattern);
    }

    patterns
}

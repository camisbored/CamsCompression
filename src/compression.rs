use std::fs::File;
use std::io::{Read, Write};

use super::pattern::Pattern;

use super::utils::get_byte_count_from_int;
use super::utils::get_byte_series_from_int;

const TARGET_OCCURANCE_TIMES : usize = 8;

/// compress
/// the main function called to compress a file
/// takes input path as as string, and output path as string
pub fn compress(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;

    let mut patterns: Vec<Pattern> = Vec::new();
    let mut compressed_data: Vec<u8> = Vec::new();

    //read in entire file
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    //detect long sequences of same character. if present, add as pattern
    //otherwise write to output buffer
    let mut i: usize = 0;
    while i < buffer.len(){
                let mut count = 1;
                while i + count < buffer.len() && buffer[i] == buffer[i + count] {
                    count+=1;
                }

                if count >= TARGET_OCCURANCE_TIMES {
                    let pattern = Pattern::new_serialize(
                        i.try_into().unwrap(),
                        count.try_into().unwrap(),
                        buffer[i],
                    );
                    patterns.push(pattern);
                    i += count;
                } else {
                	compressed_data.push(buffer[i]);
                    i+=1;
                }
    }

    //write each pattern, if any, to table buffer
    let mut table_data: Vec<u8> = Vec::new();
    let mut is_table_present  = false;
    for p in &patterns{
        is_table_present = true;
        if p.bytes_to_desc_addr == 0{
            table_data.push(1);
        } else {
            table_data.push(p.bytes_to_desc_addr);
        }
        table_data.extend(get_byte_series_from_int(p.addr)?);
        table_data.push(p.bytes_to_desc_count.try_into().unwrap());
        table_data.extend(get_byte_series_from_int(p.count)?);
        table_data.push(p.character.try_into().unwrap());

    }

    let mut header_data: Vec<u8> = Vec::new();
    //write magic number 'C' 'G'
    header_data.push(0x43);
    header_data.push(0x47);

    //write remainded of header
    if is_table_present {
        let mut table_address_description = get_byte_count_from_int(compressed_data.len().try_into().unwrap());
        if table_address_description == 0 {
            table_address_description = 1;
        }
        header_data.push(table_address_description);
        header_data.extend( get_byte_series_from_int(compressed_data.len().try_into().unwrap())?);
    } else {
        //if no pattern detected, write 0x00 to bytes 2 and 3 to empty pattern table
        header_data.push(0x00);
        header_data.push(0x00);
    }

    //write all buffers to output file
    output_file.write_all(&header_data)?;
    output_file.write_all(&compressed_data)?;
    output_file.write_all(&table_data)?;

    Ok(())
}
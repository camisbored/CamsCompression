use std::fs::File;
use std::io::{Read, Write};

use super::pattern::Pattern;

//use super::utils::expand_vec;
use super::utils::get_int_from_vec;
use super::utils::parse_patterns;

/// decompress
/// the main function called to decompress a file
/// takes input path as as string, and output path as string
pub fn decompress(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;
    let mut output_buffer = Vec::new();

    //load whole inupt file into buffer
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    //parse header to ensure valid CG file, exit and return error if reqs not met
    if buffer.len() < 4 || buffer[0] != b'C' || buffer[1] != b'G' {
        panic!("Invalid file format (missing CG header)");
    }

    let mut is_header_present = true;
            
    if buffer[2]==0 && buffer[3]==0 {
        is_header_present = false;
    }

    //remove magic number from file
    buffer.drain(..2);

    //if there is data in the pattern table
    if is_header_present{

        //load first byte to describe table address, remove from buffer
        let byte_to_desc_table_addr = buffer[0];
        buffer.remove(0);

        //load address of pattern table, remove rest of header
        let addr_bytes = buffer[..4].to_vec();
        let table_addr = get_int_from_vec(byte_to_desc_table_addr, &addr_bytes);
        buffer.drain(..byte_to_desc_table_addr as usize);

        //load table from the end of file, remove from buffer
        let pattern_data = buffer[table_addr..=buffer.len()-1].to_vec();
        buffer.drain(table_addr..=buffer.len()-1);

        let mut total_output_size: u32 = buffer.len() as u32;
        let mut file_pointer = 0;
        let mut old_file_pointer = 0;

        //for each pattern, insert data back into file
        let patterns: Vec<Pattern> = parse_patterns(&pattern_data);
        for pattern in &patterns {
            while file_pointer < pattern.addr{
                output_buffer.push(buffer[old_file_pointer as usize]);
                file_pointer+=1;
                old_file_pointer+=1;
            }
            for _ in 0..pattern.count {
                output_buffer.push(pattern.character);
            }
            file_pointer+=pattern.count;
            total_output_size += pattern.count;
        }

        while file_pointer < total_output_size{
            output_buffer.push(buffer[old_file_pointer as usize]);
            file_pointer+=1;
            old_file_pointer+=1;
        }

        //write completed buffer to output file
        output_file.write_all(&output_buffer)?;
    } else {
        //if there was no pattern data, remove remainder of header, initial file is then in tack.
        buffer.drain(..2);
        output_file.write_all(&buffer)?;

    }

    Ok(())
}
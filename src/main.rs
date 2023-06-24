/// @Grande Software Solutions, 2023
/// cams_compression
/// This is a rust implementation of a custom compression algorithm, focusing on removing
/// padding/repeated bytes.
/// 
/// The format for this compressed format (we will call .cg files) consists of:
/// 
/// Bytes [0, 1] = 0x43, 0x47
/// Byte  [2] = unsigned char [1-4] describing how many bytes to read for address
/// The following 1-4 bytes contain the address of a table at the end of the file.
/// Following the address is the compressed data.
/// After all the compressed data, sits a table with 1 or more entries
/// Entry byte [0] = unsigned char [1-4] describing how many bytes to read to describe "start index"
/// The next 1-4 bytes describe the address of the start index in big endian
/// The next byte is an unsigned char describing how many bytes to read to describe the "count"
/// The next 1-4 bytes is how many times pattern occurs as a big endian integer.
/// The final byte in a table entry is the actual byte value that was repeated
/// Ex: {0x02, 0x0512, 0x04, 0x08432543, 0xFF} 
/// means the byte 0xFF occured at address 0x512 0x08432543 times.

use std::env;

mod pattern;
mod utils;
mod compression;
mod decompression;

use compression::compress;
use decompression::decompress;
use utils::print_help;

/// main
/// driver code to process arguments and run compression/decompression
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        print_help();
        return;
    }

    //parse arguments
    let operation = &args[1];
    let input_path = &args[2];
    let output_path = &args[3];

    //determine if we are compressing or decompressing, or if else print help
    match operation.as_str() {
        "-c" => {
            if let Err(err) = compress(input_path, output_path) {
                eprintln!("Compression failed: {}", err);
            } else {
                println!("Compression completed successfully.");
            }
        }
        "-d" => {
            if let Err(err) = decompress(input_path, output_path) {
                eprintln!("Decompression failed: {}", err);
            } else {
                println!("Decompression completed successfully.");
            }
        }
        _ => {
            println!("Invalid operation.");
            utils::print_help();
        }
    }
}
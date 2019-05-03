extern crate ccsds_primary_header;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use bytes::Bytes;
use bytes::BytesMut;
use core::option::Option;

/// # parse_input() function
/// `parse_input()` should accept an object representing a
/// loaded CCSDS compliant data file, create an instance of
/// a CCSDS Header Parser, process the data, and return the
/// header to main in a useful data format
///

fn parse_input(bytestream: Bytes) -> Option<BytesMut> {
    let mut parser = ccsds_primary_header::parser::CcsdsParser::new();      
    parser.recv_bytes(bytestream);

    let header = parser.current_header().unwrap();
    
    println!("Primary Header: {:?}", header);
    println!("apid: {:?}", header.control.apid());
    
    return parser.pull_packet();
}
    


/// # main() function
/// `main()` should accept an input data file and
/// appropriately pass it to a function which contains
/// an instation of the CCSDS Header Parser.  This should
/// return the parsed header in a useful data format.
///

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    println!("Loading data files {:?}...", args);

    let file = match File::open(&args[0]) {
        Err(e) => {
            println!("Error opening file: {}", e);
            return Ok(());
        }
        Ok(f) => f,
    };
        
        
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::<u8>::new();

    reader.read_to_end(&mut buffer);

    let mem = Bytes::from(buffer);
    let data = parse_input(mem);

    println!("Extracted packet: {:?}", data);

    println!("Program terminated sucessfully!");
    Ok(())
}

extern crate ccsds_primary_header;

use std::env;
use std::fs;
use ccsds_primary_header::primary_header::*;
use ccsds_primary_header::parser::*;

/// # parseInput() function
/// `parseInput()` should accept an object representing a
/// loaded CCSDS compliant data file, create an instance of
/// a CCSDS Header Parser, process the data, and return the
/// header to main in a useful data format
///

fn parseInput(bytestream: &Vec<u8>) {
    let parser = parser::CcsdsParser.new();    
    
    for val in bytestream {
        parser.recv_bytes(val)
    };
}


/// # main() function
/// `main()` should accept an input data file and
/// appropriately pass it to a function which contains
/// an instation of the CCSDS Header Parser.  This should
/// return the parsed header in a useful data format.
///

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Loading data files {:?}...", args);

    let f = File::open(args[0])?;
    let mut buffer = Vec<u8>::new();

    f.read_to_end(&mut buffer)?;

    parseInput(&buffer);

    println!("Program terminated sucessfully!")
}

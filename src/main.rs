extern crate ccsds_primary_header;

use std::env;
use std::str;
use std::fs::File;
use std::io::prelude::*;
use bytes::Bytes;

/// # parse_input() function
/// `parse_input()` should accept an object representing a
/// loaded CCSDS compliant data file, create an instance of
/// a CCSDS Header Parser, process the data, and return the
/// header to main in a useful data format
///

fn parse_input(bytestream: bytes::Bytes) -> ccsds_primary_header::parser::CcsdsParser {
    let mut parser = ccsds_primary_header::parser::CcsdsParser::new();      
    parser.recv_bytes(bytestream);

    let header = parser.current_header().unwrap();
                 
    return parser;
}
    
fn read_header(header: &ccsds_primary_header::primary_header::CcsdsPrimaryHeader) -> String {

    let mut stringbuf = String::new();
    
    stringbuf.push_str("#####################\n");
    
    stringbuf = stringbuf + format!("# Packet Type: {:?} # APID: {:?} # Secondary Header?: {:?} # \n",
                    header.control.packet_type(), header.control.apid(), header.control.secondary_header_flag()).as_str();

    stringbuf = stringbuf + format!("# Sequence Type: {:?} # Sequence Count: {:?} # Length Field: {:?} # \n",
                    header.sequence.sequence_type(), header.sequence.sequence_count(), header.length.length_field()).as_str();
    
    stringbuf.push_str("#####################\n");

    return stringbuf;
}


/// # main() function
/// `main()` should accept an input data file and
/// appropriately pass it to a function which contains
/// an instation of the CCSDS Header Parser.  This should
/// return the parsed header in a useful data format.
///

fn main() -> Result<(), std::io::Error> {

    // Collect command line arguments and attempt to
    // open the first file listed
    
    let args: Vec<String> = env::args().collect();
    println!("Loading data files {:?}...\n", args);

    println!("Initial Arg: {:?}",&args[1]);
    
    let mut file = match File::open(&args[1]) {
        Err(e) => {
            println!("Error opening file: {}", e);
            return Ok(());
        }
        Ok(f) => f,
    };

    // Create a buffer in the form of a vector
    // containing 8-bit unsigned integers and use
    // File structs read trait to populate via
    // mutable reference
    
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer);

    // The CCSDS parser crate manipulates bytes::Bytes structs.
    // Therefore, it is necessary to populate one of these with the
    // contents of the Vec<u8> buffer
    
    let mut mem = Bytes::from(buffer);

    // Now that we've handled the input, handle the output.
    //
    // Create a new File object using the name of the loaded
    // file.  Filename should be the name of the input file
    // with the suffix '.txt'

    let filename = format!("{}",&args[1]);
    println!("Filename is: {}",filename);
    let mut outfile = File::create(filename.trim_end_matches(".dat").to_owned()+".txt").unwrap();
    
    // Call Parse input to return a parser loaded with
    // the passed byte string.  Parser object can then
    // be manipulated to retrive data from the packaged
    // stream.

    let mut data = parse_input(mem);

    // output is a string struct which will be filled with
    // the parsed packet data
    
    let mut output = String::new();

    // This loop calls read_header() and pushs the results to the
    // output string.  Loop termination occurs when there are no
    // more packets to pull.
    
    loop {
        let pulled = &data.pull_packet(); 
        if pulled.is_some() == false {
            println!("End of data reached!");
            break;
        } else {
            output.push_str(read_header(&data.current_header().unwrap()).as_str());
        }
    };

    outfile.write_all(output.as_bytes());
    println!("Program terminated sucessfully!");
    Ok(())   
}

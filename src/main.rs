use std::env;
use std::fs::File;
use std::string::String;

use std::io::prelude::Read;

const BUF_SIZE: usize = 1 * 1024 * 1024;

const THREAD_START: &str = "<div class=\"thread\">";
const MESSAGE_START: &str = "<div class=\"message\"><div class=\"message_header\"><span class=\"user\">";

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: & String = &args[1];

    let mut file = File::open(filename).expect("File not found");
    let metadata = file.metadata().expect("Failed to read metadata from fiel");
    let file_size = metadata.len();
    //let reader = BufReader::new(file);

    let mut buffer = vec![0; BUF_SIZE];
    let bytes_read = file.read(&mut buffer).expect("Failed to read from file");
    //let read_string = String::from_utf8(buffer).expect("Failed to create string from bytes");

    let thread_start: &[u8] = THREAD_START.as_bytes();
    let message_start: &[u8] = MESSAGE_START.as_bytes();

    let mut thread_count: usize = 0;
    let mut message_count: usize = 0;
    
    let byte_iterator = buffer.iter();
    let thread_starts = byte_iterator.enumerate()
        .filter(|&(_,&c)| c == thread_start[0])
        .filter(|&(i,&_)| thread_start.iter().zip(buffer[i..].iter()).all(|(a,b)| a == b))
        .map(|(i,_)| i);

    for thread_start in thread_starts {
        //println!("Found match at: {}", thread_start);
        let thread_messages = buffer[thread_start..].iter().enumerate()
            .filter(|&(_,&c)| c == message_start[0])
            .filter(|&(i,&_)| message_start.iter().zip(buffer[i..].iter()).all(|(a,b)| a == b))
            .map(|(i,_)| i).count();
        message_count += thread_messages;
        thread_count += 1;
    }

    println!("{:10} / {}", thread_count, message_count);
    println!("{:10} / {} :: {:5.2}%",
             bytes_read,
             file_size,
             (bytes_read as f64 * 100.0/file_size as f64));
    //println!("{:?}", String::from_utf8(buffer));
}

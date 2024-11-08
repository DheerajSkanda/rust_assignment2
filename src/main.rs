// serialization program
use protobuf::Message;
use std::{fs::File, io::{self, Read, Write}};
use clap::{Arg, Command};
mod person;
use person::Person;

fn main() -> io::Result<()> {
    let matches = Command::new("protobuf-example")
        .about("A simple program to read input from a file and write output to another file")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_parser(clap::value_parser!(String))
                .help("Input file containing person data (last_name, first_name, date_of_birth)")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_parser(clap::value_parser!(String))
                .help("Output file to write serialized data")
                .required(true),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();

    let mut file = File::open(input_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{:?}",contents);

    let mut output = File::create(output_file)?;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 3 {
            let mut person = Person::new();
            person.set_last_name(parts[0].to_string());
            person.set_first_name(parts[1].to_string());
            person.set_date_of_birth(parts[2].to_string());

            let person_data = person.write_to_bytes().expect("Failed to serialize person");

            let len = person_data.len() as u32;
            output.write_all(&len.to_le_bytes())?; // Write length as 4 bytes (little-endian)
            output.write_all(&person_data)?; // Write serialized data
            println!("  len:{:?}",len);
            println!("Serialized: {:?}", person_data);
        } else {
            eprintln!("Skipping invalid line: {}", line);
        }
    }

    println!("Serialized person data saved to {}", output_file);
    Ok(())
}

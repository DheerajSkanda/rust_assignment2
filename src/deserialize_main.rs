// deserialization program
use protobuf::Message;
use std::{fs::File, io::{self, Read, Write}};
use clap::{Arg, Command};
mod person;
use person::Person;

fn main() -> io::Result<()> {
    let matches = Command::new("protobuf-example")
        .about("Deserialize binary protobuf data into readable text")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_parser(clap::value_parser!(String))
                .help("Input file containing serialized protobuf data")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_parser(clap::value_parser!(String))
                .help("Output file to write deserialized data")
                .required(true),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();

    let mut file = File::open(input_file)?;
    let mut output = File::create(output_file)?;

    loop {
        let mut len_buf = [0; 4];
        if file.read_exact(&mut len_buf).is_err() {
            break; // End of file reached
        }
        let len = u32::from_le_bytes(len_buf) as usize;
        println!("len:{:?}",len);

        let mut person_data = vec![0; len];
        if file.read_exact(&mut person_data).is_err() {
            eprintln!("Failed to read person data of expected length");
            break;

        }
        println!("person_data:{:?}",person_data);
        match Person::parse_from_bytes(&person_data) {
            Ok(person) => {
                let line = format!(
                    "{},{},{}\n",
                    person.last_name,
                    person.first_name,
                    person.date_of_birth
                );
                output.write_all(line.as_bytes())?;
                println!("{:#?}",line);
            }
            Err(e) => {
                eprintln!("Failed to deserialize person: {:?}", e);
                break;
            }
        }
    }

    println!("Deserialized person data saved to {}", output_file);
    Ok(())
}

use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;
use uniquely_decodable::schlinkert::is_uniquely_decodable;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Read file_path as {}", file_path);
    let wordlist: Vec<String> =
        read_by_line(file_path.into()).expect("Unable to read inputted file");

    println!("Word list has {} words", wordlist.len());

    println!("Uniquely decodable? {}", is_uniquely_decodable(&wordlist));
}

fn read_by_line<T: FromStr>(file_path: PathBuf) -> io::Result<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut vec = Vec::new();
    let f = match File::open(file_path) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line?.parse() {
            Ok(l) => vec.push(l),
            Err(e) => panic!("Error parsing line from file: {:?}", e),
        }
    }
    Ok(vec)
}

use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;
use uniquely_decodable::*;

fn criterion_benchmark(c: &mut Criterion) {
    // Define a Criterion group, just so we can set a sample_size and significance_level
    let mut group = c.benchmark_group("Check short UD list");
    group.sample_size(100).significance_level(0.1);

    let ud_list: Vec<String> = read_by_line("./benches/lists/short-ud-list.txt".into()).unwrap();

    group.bench_function("Schlinkert", |b| {
        b.iter(|| schlinkert::is_uniquely_decodable(&ud_list))
    });

    // group.bench_function("Colfenor-Rodeh", |b| {
    //     b.iter(|| colfenor_rodeh::is_uniquely_decodable(&ud_list))
    // });

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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

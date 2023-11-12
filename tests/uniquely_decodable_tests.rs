mod uniquely_decodable_tests {
    use uniquely_decodable::colfenor_rodeh;
    use uniquely_decodable::schlinkert;

    #[test]
    fn can_determine_if_a_short_list_is_uniquely_decodable() {
        let code: Vec<String> = vec![
            "101".to_string(),
            "00".to_string(),
            "0001".to_string(),
            "1".to_string(),
        ];
        assert!(schlinkert::is_uniquely_decodable(&code));
        assert!(colfenor_rodeh::is_uniquely_decodable(&code));
    }

    #[test]
    fn can_determine_if_a_short_list_is_not_uniquely_decodable() {
        let code: Vec<String> = vec![
            "car".to_string(),
            "pet".to_string(),
            "carpet".to_string(),
            "convertible".to_string(),
        ];
        assert!(!schlinkert::is_uniquely_decodable(&code));
        assert!(!colfenor_rodeh::is_uniquely_decodable(&code));
    }

    #[test]
    fn can_determine_if_a_longer_list_free_of_prefix_words_is_uniquely_decodable() {
        let list: Vec<String> =
            read_by_line("./tests/test-files/ud/eff-sample.txt".into()).unwrap();

        assert!(schlinkert::is_uniquely_decodable(&list));
        assert!(colfenor_rodeh::is_uniquely_decodable(&list));
    }

    #[test]
    fn can_determine_if_bips_list_is_not_uniquely_decodable() {
        let list: Vec<String> = read_by_line("./tests/test-files/not-ud/bips.txt".into()).unwrap();

        assert!(!schlinkert::is_uniquely_decodable(&list));
        assert!(!colfenor_rodeh::is_uniquely_decodable(&list));
    }
    use std::fs::File;
    use std::io;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::path::PathBuf;
    use std::str::FromStr;

    /// Generatic function that reads a file in, line by line.
    /// Not sure if all of this is necessary, but it gets the job done.
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

fn main() {
    let alphabet1: Vec<String> = vec!["0".to_string(), "10".to_string(), "110".to_string()]; // uniquely decodable
    let alphabet2: Vec<String> = vec!["1".to_string(), "11".to_string(), "10".to_string()]; // 1110 can be 11|10 OR 1|1|10; so NOT uniquely decodable
    let alphabet3: Vec<String> = vec!["am".to_string(), "amber".to_string(), "bullet".to_string()];
    let alphabet4: Vec<String> = vec!["boy".to_string(), "hood".to_string(), "boyhood".to_string()];
    let alphabet5: Vec<String> = vec![
        "spill".to_string(),
        "sun".to_string(),
        "moved".to_string(),
        "spills".to_string(),
        "unmoved".to_string(),
    ];

    // for word in alphabet1 {
    //     println!("{}", word);
    // }
    assert!(naive_uniquely_decodable(alphabet1));
    assert!(!naive_uniquely_decodable(alphabet2));
    assert!(naive_uniquely_decodable(alphabet3));
    assert!(!naive_uniquely_decodable(alphabet4));
    assert!(!naive_uniquely_decodable(alphabet5));
}

fn naive_uniquely_decodable(list: Vec<String>) -> bool {
    let mut list_plus_suffixes: Vec<String> = list.clone();
    // for potential_prefix_word in &list {
    for i in 0..list_plus_suffixes.len() {
        let potential_prefix_word = list_plus_suffixes[i].to_owned();
        // for word in &list {
        for j in 0..list_plus_suffixes.len() {
            let word = list_plus_suffixes[j].to_owned();

            match get_dangling_suffix_if_exists(&potential_prefix_word, &word) {
                Some(dangling_suffix) => {
                    println!(
                        "Found a DS: {} ({} and {})",
                        dangling_suffix, potential_prefix_word, word
                    );
                    if list_plus_suffixes.contains(&dangling_suffix) {
                        println!(
                            "'bout to return false cuz {:?} contains {}",
                            list_plus_suffixes, dangling_suffix
                        );
                        return false;
                    } else {
                        list_plus_suffixes.push(dangling_suffix);
                    }
                }
                None => continue, // not sure
            }
        }
    }
    println!("full list is {:?}", list_plus_suffixes);
    true
}

fn get_dangling_suffix_if_exists(word1: &str, word2: &str) -> Option<String> {
    if word1 == word2 {
        return None;
    }
    let mut dangling_suffix: Option<String> = None;
    if word1.starts_with(word2) {
        println!("in first if");
        let shorter_word_length = word2.chars().count();
        dangling_suffix = Some(word1.get(shorter_word_length..).unwrap().to_string());
        // } else if word2.starts_with(word1) {
        //     let shorter_word_length = word1.chars().count();
        //     dangling_suffix = Some(word2.get(shorter_word_length..).unwrap().to_string());
    }

    return dangling_suffix;
}

pub mod sardinas_patterson;
use crate::sardinas_patterson::is_uniquely_decodable;

fn main() {
    let code: Vec<String> = vec![
        "101".to_string(),
        "00".to_string(),
        "0001".to_string(),
        "1".to_string(),
    ];
    assert!(is_uniquely_decodable(&code));
}

pub mod colfenor_rodeh;
use crate::colfenor_rodeh::is_uniquely_decodable;

fn main() {
    let code: Vec<String> = vec![
        "101".to_string(),
        "00".to_string(),
        "0001".to_string(),
        "1".to_string(),
    ];
    assert!(is_uniquely_decodable(&code));

    let code: Vec<String> = vec![
        "car".to_string(),
        "pet".to_string(),
        "carpet".to_string(),
        "convertible".to_string(),
    ];
    assert!(!is_uniquely_decodable(&code));
}

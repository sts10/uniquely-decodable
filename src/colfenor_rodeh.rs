/* sardinas patterson algorithm for testing unique decipherability */
// reference: IEEE TRANSACTIONS ON INFORMATION THEORY, VOL. IT-28, NO. 4, JULY 1982
// https://github.com/Colfenor/sardinas-patterson

fn duplicates_inside(vector: &Vec<String>) -> bool {
    let mut vector_copy = vec![];

    for x in vector {
        if vector_copy.contains(x) {
            return true;
        }
        vector_copy.push(String::from(x));
    }
    false
}

pub fn is_uniquely_decodable(codeword_list: &Vec<String>) -> bool {
    let mut tails = vec![];

    //E1.1 check for duplicates in our list of codewords
    if duplicates_inside(codeword_list) {
        // println!("duplicate word detected \n");
        return false;
    }

    // E1.2
    for i in codeword_list {
        for j in codeword_list {
            if i != j && i.chars().count() > j.chars().count() && i.find(j) == Some(0) {
                // E1.1
                //todo extract suffix and save in vector
                let suffix = &i[j.chars().count()..];
                tails.push(String::from(suffix));
                // println!("{}\n", suffix);
            }
        }
    }

    // E2
    let mut i = 0;
    while i < tails.len() {
        for j in codeword_list {
            if &tails[i] == j {
                return false;
            }
            let mut sigma = String::new();

            if tails[i].chars().count() > j.chars().count() && tails[i].find(j) == Some(0) {
                sigma = tails[i][j.chars().count()..].to_owned();
            } else if tails[i].chars().count() < j.chars().count() && j.find(&tails[i]) == Some(0) {
                sigma = j[tails[i].chars().count()..].to_owned();
            }

            let mut tail_concat = tails[i].to_owned();
            let mut word_concat = j.to_owned();

            word_concat.push_str(&sigma);
            tail_concat.push_str(&sigma);

            if &tail_concat == j || word_concat == tails[i] {
                tails.push(sigma);
            }
        }
        i += 1;
    }
    true
}

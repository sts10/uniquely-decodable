/* sardinas patterson algorithm for testing unique decipherability */
// reference: IEEE TRANSACTIONS ON INFORMATION THEORY, VOL. IT-28, NO. 4, JULY 1982
// https://github.com/Colfenor/sardinas-patterson
// With improvements from @westonal

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
            if i != j && i.len() > j.len() && i.starts_with(j) {
                // E1.1
                //todo extract suffix and save in vector
                let suffix = &i[j.len()..];
                tails.push(suffix);
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
            let sigma = if tails[i].len() > j.len() && tails[i].starts_with(j) {
                &tails[i][j.len()..]
            } else if tails[i].len() < j.len() && j.starts_with(&tails[i]) {
                &j[tails[i].len()..]
            } else {
                ""
            };

            let mut tail_concat = tails[i].to_string();
            let mut word_concat = j.to_string();

            word_concat.push_str(sigma);
            tail_concat.push_str(sigma);

            if &tail_concat == j || word_concat == tails[i] {
                tails.push(sigma);
            }
        }
        i += 1;
    }
    true
}

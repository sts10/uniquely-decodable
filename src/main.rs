// Following along with https://github.com/danhales/blog-sardinas-patterson/blob/master/index.ipynb
fn main() {
    let c = vec!["a".to_string(), "an".to_string(), "apple".to_string()];
    // let c = vec!["02".to_string(), "12".to_string(), "120".to_string(), "20".to_string(), "21".to_string()];
    println!("c1 is {:?}", generate_c1(c));
    assert!(
        generate_c1(vec!["a".to_string(), "an".to_string(), "apple".to_string()])
            == vec!["n".to_string(), "pple".to_string()]
    );

    // Now let's try this fancier generate_cn function
    let c = vec!["0".to_string(), "011".to_string(), "01".to_string()];
    println!("For the set C = {:?}", c);
    for i in 0..10 {
        println!("C_{:?} is {:?}", i, generate_cn(&c, i));
    }

    // Now... to infinity?
    assert_eq!(
        generate_c_infinity(&c),
        vec!["1".to_string(), "11".to_string()]
    );

    let c = vec![
        "02".to_string(),
        "12".to_string(),
        "120".to_string(),
        "20".to_string(),
        "21".to_string(),
    ];
    println!("--------------");
    // This repeats infinitly!
    //generate_c_infinity(&c);
    // lets halt if it gets too repetitive
    assert!(if_vector_is_sub_vector(
        &["1".to_string(), "2".to_string(), "3".to_string()],
        &["1".to_string()]
    ));

    let final_set = generate_c_infinity_with_a_halt_break(&c);
    println!("Final: {:?}", final_set);
    println!("--------------");
    check_decodability(&c);
}

fn generate_c1(c: Vec<String>) -> Vec<String> {
    let mut c1 = vec![];
    for w1 in &c {
        for w2 in &c {
            if w1.len() > w2.len() && w1.starts_with(w2) {
                // w2 is a prefix word of w1
                //println!("about to push. w1 is {:?} and w2 is {:?}", w1, w2);
                // so, we're goign to add the dangling suffix to a new vector
                // called c1
                c1.push(w1[w2.len()..].to_string());
            }
        }
    }
    c1
}

// generate c for any number n
fn generate_cn(c: &[String], n: usize) -> Vec<String> {
    if n == 0 {
        return c.to_vec();
    } else {
        let mut cn = vec![];

        // generate c_(n-1)
        let cn_minus_1 = generate_cn(c.clone(), n - 1);
        for w1 in c {
            for w2 in &cn_minus_1 {
                if w1.len() > w2.len() && w1.starts_with(w2) {
                    // w2 is a prefix word of w1
                    // so, we're goign to add the dangling suffix to a new vector
                    // called c1
                    cn.push(w1[w2.len()..].to_string());
                }
            }
        }
        // Now the other way?
        for w1 in cn_minus_1 {
            for w2 in c {
                if w1.len() > w2.len() && w1.starts_with(w2) {
                    // w2 is a prefix word of w1
                    // so, we're goign to add the dangling suffix to a new vector
                    // called c1
                    cn.push(w1[w2.len()..].to_string());
                }
            }
        }
        cn.sort();
        cn.dedup();
        return cn;
    }
}

fn generate_c_infinity(c: &[String]) -> Vec<String> {
    let mut c_infinity = vec![];
    let mut n = 1;
    let mut cn = generate_cn(c, n);

    while cn.len() > 0 {
        c_infinity.append(&mut cn);
        c_infinity.sort();
        c_infinity.dedup();
        n = n + 1;
        cn = generate_cn(c, n);
        println!("n ={:?} {:?}", n, c_infinity);
    }
    return c_infinity;
}

fn generate_c_infinity_with_a_halt_break(c: &[String]) -> Vec<String> {
    let mut cs = vec![];
    let mut c_infinity = vec![];
    let mut n = 1;
    let mut cn = generate_cn(c, n);
    println!("c_{:?} {:?}", n, cn);

    while cn.len() > 0 {
        cn.sort();
        cs.sort();
        cs.dedup(); // a little worried about this...
        println!("About compare {:?} and {:?}", cn, cs);
        if if_vector_is_sub_vector(&cs, &cn) {
            println!("Cycle detected. Halting algorithm.");
            break;
        } else {
            // println!("Before append, cn is {:?}", cn);
            // Can't use append here because that drains one of the Vectors
            cs = concat_vectors_expensively(cs, &cn);
            //  println!("After append, cn is {:?}", cn);
            c_infinity.append(&mut cn.to_vec());
            c_infinity.sort();
            c_infinity.dedup();
            n = n + 1;
            cn = generate_cn(c, n);
            println!("c_{:?} {:?}", n, c_infinity);
        }
    }
    return c_infinity;
}

fn if_vector_is_sub_vector(big_vec: &[String], little_vec: &[String]) -> bool {
    let footprint_size = little_vec.len();
    if footprint_size == 0 || big_vec.len() == 0 || big_vec.len() < little_vec.len() {
        return false;
    }
    if little_vec == big_vec {
        return true;
    }
    for chunk in big_vec.windows(footprint_size) {
        if chunk == little_vec {
            return true;
        }
    }
    return false;
}

fn concat_vectors_expensively(mut v1: Vec<String>, v2: &[String]) -> Vec<String> {
    let v1_temp = &mut v1;
    let mut v2_temp = v2.to_vec();
    v1_temp.append(&mut v2_temp);
    v1_temp.to_vec()
}

/// Returns true if c in uniquely decodable
fn sardinas_patterson_theorem(c: &[String]) -> bool {
    let c_infinity = generate_c_infinity_with_a_halt_break(&c);
    let num = budget_intersection_count(c, &c_infinity);
    return num == 0;
}

fn budget_intersection_count(v1: &[String], v2: &[String]) -> usize {
    let mut overlap = 0;
    for element in v1 {
        if v2.contains(element) {
            overlap = overlap + 1;
        }
    }
    overlap
}

fn check_decodability(c: &[String]) {
    if sardinas_patterson_theorem(&c) {
        println!("{:?} is uniquely decodable", c);
    } else {
        println!("{:?} is NOT uniquely decodable", c);
    }
}

//! This is a (rather clumsily) implementation of the Sardinas-Patterson algorithm
//! by Sam Schlinkert.
//! The goal is to check if a word list (`c`) is uniquely decodable.
//!
//! I followed
//! <https://github.com/danhales/blog-sardinas-patterson/blob/master/index.ipynb>
//! very closely. Since then, other contributors have refactored it.
use dashmap::DashSet;

/// Return true if the list is uniquely decodable, false if not. I
/// don't _think_ we need to check reversed words in this case.
pub fn is_uniquely_decodable<T: AsRef<str>>(c: &[T]) -> bool {
    sardinas_patterson_theorem(&c.iter().map(|f| f.as_ref()).collect())
}

/// Generate c for any number n
fn generate_cn<'a>(c: &DashSet<&'a str>, cn_minus_1: &DashSet<&'a str>) -> DashSet<&'a str> {
    let cn = DashSet::new();

    for w1 in c.iter() {
        for w2 in cn_minus_1.iter() {
            if w1.len() > w2.len() && w1.starts_with(*w2) {
                // w2 is a prefix word of w1
                // so, we're going to add the dangling suffix to a new DashSet
                // called cn
                cn.insert(&w1[w2.len()..]);
            }
            if w2.len() > w1.len() && w2.starts_with(*w1) {
                // w1 is a prefix word of w2
                // so, we're going to add the dangling suffix to a new DashSet
                // called cn
                cn.insert(&w2[w1.len()..]);
            }
        }
    }
    cn
}

fn generate_c_infinity_with_a_halt_break<'a>(c: &DashSet<&'a str>) -> DashSet<&'a str> {
    let mut cn = generate_cn(c, c);
    let mut cs = cn.clone();

    loop {
        cn = generate_cn(c, &cn);
        let prior = cs.len();
        cs.extend(cn.iter().map(|s| *s));
        if cs.len() == prior {
            // if the set size did not increase, cn is a subset
            // Cycle detected. Halting algorithm.
            break;
        }
    }
    cs
}

/// Returns true if c is uniquely decodable
fn sardinas_patterson_theorem(c: &DashSet<&str>) -> bool {
    let c_infinity = generate_c_infinity_with_a_halt_break(c);
    if c.len() <= c_infinity.len() {
        c.iter().all(|v| !c_infinity.contains(*v))
    } else {
        c_infinity.iter().all(|v| !c.contains(*v))
    }
}

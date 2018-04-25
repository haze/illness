
use rand::{thread_rng, Rng, ThreadRng};

pub fn random_string(length: u64) -> String {
    let mut rng: ThreadRng = thread_rng();
    let mut gen = rng.gen_ascii_chars();
    let mut buff = String::new();
    for _ in 0 .. length {
        let next = gen.next();
        if next.is_none() {
            return buff
        }
        buff.push(next.unwrap());
    }
    buff
}
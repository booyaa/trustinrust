extern crate rand;

use rand::{thread_rng, Rng};

pub fn handle(req : String) -> String {
    let mut rng = thread_rng();
    let split = req.split(",");
    let mut vec: Vec<&str> = split.collect();
    rng.shuffle(&mut vec);
    let joined = vec.join(", ");
    format!("{:?}", joined)
}



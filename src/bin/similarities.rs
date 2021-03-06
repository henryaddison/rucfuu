extern crate rucommender;

use std::error::Error;
use std::process;

use rucommender::similarities;

fn main() {
    let sims = match similarities(std::io::stdin()) {
        Ok(sims) => {
            sims
        },
        Err(err) => {
            handle_err(err);
            return ();
        }
    };

    println!("{:?}", sims);

    ()
}

fn handle_err(err: Box<Error>) {
    println!("{}", err);
    process::exit(1);
}

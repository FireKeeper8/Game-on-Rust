use piston_window::*;

use crate ::libm::music;
mod libm;

fn main() {

	match music() {
        Ok(_) =>(),
        Err(err) => println!("Error: {}", err),
    }
    
}

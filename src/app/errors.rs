use bevy::prelude::*;

pub fn print_error(In(result): In<Result<(), Box<dyn std::error::Error>>>) {
    if let Err(e) = result {
        println!("Error: {:?}", e);
    }
}

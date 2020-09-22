extern crate chrono; 

use chrono::prelude::*;

fn main() {

    let local_now: DateTime<Local> = Local::now();

    println!("{}", local_now);

}
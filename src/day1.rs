use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let filename = "data/1.txt";

    println!("Read from file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut total = 0;
    let mut found = false;
    let mut round = 0;

    let mut map = HashMap::new();
    map.insert(0, true);

    while !found {
        for i in contents.split_whitespace() {
            let value = i.replace('+', "")
                .parse::<i32>().unwrap();

            total += value;

            if map.contains_key(&total) && !found {
                println!("Have been here already: {}", total);
                found = true;
            }
            map.insert(total, true);
        }
        round += 1;
        if round == 1 {
            println!("Total: {}", total);
        }
    }
}
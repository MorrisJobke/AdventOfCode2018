use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let filename = "data/12-example.txt";

    println!("Read from file {}", filename);
    let now = Instant::now();

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");


    let mut line_number = 0;
    let mut plants = String::from("....................");
    let mut patterns = HashMap::new();
    let offset = 20;
    let mut min = 0;
    let mut max = 0;
    let debug = true;

    for line in contents.split("\n") {
        if line_number == 0 {
            plants.push_str(&line[15..]);
            max = plants.len() as i32;
            plants.push_str("..........................");
        } else if line_number > 1 {
            if &line[5..10] == " => ." {
                continue;
            }
            patterns.insert(&line[0..5], true);
        }

        line_number += 1;
    }

    println!("Plants: \n{}", plants);
    println!("");

    println!("Finished parsing in {}.{} seconds.", now.elapsed().as_secs(), now.elapsed().subsec_millis());

    println!("                                 -         -                                        ");
    println!("                                 2         1         0         1         2         3         4         5");
    println!("                                 0         0         0         0         0         0         0         0");
    println!("generation:  0, min: {:2}, max: {} {}",  min, max, plants);

    let generations: i64 = 20;
    let now = Instant::now();

    for g in 1..(generations + 1) {
        if g < 40 || g % 10000 == 0 {
            print!("generation: {:2}, min: {:2}, max: {}", g, min, max);
        }
        let mut new_plants = plants.clone();

        for j in (min - 3)..(max + 2) {
            let start = (j + offset) as usize;
            let end = start + 5;
            let pattern = &plants[start..end];
            let position = j + 2;
            if debug { print!("{} - start: {}, end: {}, position: {}", pattern, start, end, position); }

            if patterns.contains_key(pattern) {
                if debug { println!(" - found!"); }
                new_plants.replace_range(start + 2 ..start + 3, "#");
                min = if position < min { position } else { min };
                max = if position > max { position } else { max };
            } else {
                if debug { println!(" - not found!"); }
                new_plants.replace_range(start + 2..start + 3, ".");
            }
            if end > new_plants.len() - 5 {
                new_plants.push_str(".....");
            }
        }

        min = new_plants.find('#').unwrap() as i32 - offset;

        if g < 40 {
            println!(" {}", new_plants);
        } else if g % 10000 == 0 {
            println!("");
        }
        plants = new_plants;
    }
    println!("");
    println!("Finished iterating in {}.{} seconds.", now.elapsed().as_secs(), now.elapsed().subsec_millis());

    let mut sum = 0;
    let mut position = 0 - (offset + 1);
    for c in plants.chars() {
        position += 1;
        if c == '#' {
            sum += position;
        }
    }
    println!("");
    println!("The sum is {}.", sum);
}
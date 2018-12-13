use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let filename = "data/12.txt";

    println!("Read from file {}", filename);
    let now = Instant::now();

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");


    let mut line_number = 0;
    let mut plants = String::from("....................");
    let mut patterns = HashMap::new();
    let mut offset: i64 = -20;
    let mut min: usize = -offset as usize;
    let mut max: usize = -offset as usize;
    let debug = false;

    for line in contents.split("\n") {
        if line_number == 0 {
            plants.push_str(&line[15..]);
            max = plants.len();
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

    println!("                                                              -         -                                        ");
    println!("                                                              2         1         0         1         2         3         4         5");
    println!("                                                              0         0         0         0         0         0         0         0");
    println!("generation:          0, min: {}, max: {}, offset: {:10} {}",  min, max, offset, plants);

    let generations: i64 = 20;
    let generations: i64 = 50000000000;
    let now = Instant::now();
    let mut happened_already = false;

    for g in 1..(generations + 1) {
        let old_offset = offset;
        if &plants[0..5] == "....." {
            offset += 1;
            min -= 1;
            max -= 1;
            plants = plants[1..].to_string();
        }

        if g < 40 || g % 10000 == 0 {
            print!("generation: {:10}, min: {:2}, max: {}, offset: {:10}", g, min, max, offset);
        }

        let mut new_plants = plants.clone();

        for j in (min - 3)..(max + 2) {
            let start = j as usize;
            let end = start + 5;
            let pattern = &plants[start..end];
            let position = start + 2;
            if debug { print!("{} - start: {}, end: {}", pattern, start, end); }

            if patterns.contains_key(pattern) {
                if debug { println!(" - found!"); }
                new_plants.replace_range(position..position + 1, "#");
                min = if position < min { position } else { min };
                max = if position > max { position } else { max };
            } else {
                if debug { println!(" - not found!"); }
                new_plants.replace_range(position..position + 1, ".");
            }
            if end > new_plants.len() - 5 {
                new_plants.push_str(".....");
            }
        }

        min = new_plants.find('#').unwrap();

        if g < 40 || g % 10000 == 0 {
            print!(" ");
            if offset != -20 && g % 10000 != 0 {
                for _ in 0..(offset + 20) {
                    print!(" ");
                }
            }
            println!("{}", new_plants);
        }

        if &plants[..plants.len() - 1] == &new_plants[1..] {
            if !happened_already {
                happened_already = true;
            } else {
                println!("No changes anymore after generation {} - old/new offset: {}/{}…", g, old_offset, offset);
                println!("Let's do some estimates…");
                println!("");
                println!("Pending generations: {}", generations - g);
                println!("Offset diff: {}", offset - old_offset);
                println!("");
                offset = (generations - g) * (offset - old_offset) + offset;
                println!("Estimated offset: {}", offset);
                println!("");
                plants = new_plants;
                break;
            }
        }

        plants = new_plants;
    }
    println!("");
    println!("Finished iterating in {}.{} seconds.", now.elapsed().as_secs(), now.elapsed().subsec_millis());

    let mut sum = 0;
    let mut position = offset - 1;
    for c in plants.chars() {
        position += 1;
        if c == '#' {
            sum += position;
        }
    }
    println!("");
    println!("The sum is {}.", sum);
}
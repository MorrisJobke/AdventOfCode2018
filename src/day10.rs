extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::f32::INFINITY;
use std::f32::NEG_INFINITY;

fn min(a: f32, b: f32) -> f32 {
    if b < a {
        return b;
    }
    return a;
}
fn max(a: f32, b: f32) -> f32 {
    if b > a {
        return b;
    }
    return a;
}
fn print_field(field: Vec<Vec<char>>) {
    for row in field.iter() {
        for cell in row.iter() {
            print!("{}", cell);
        }
        println!("");
    }
}

struct Point {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}


fn main() {
    let filename = "data/2.txt";

    println!("Read from file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines = contents.lines();

    let mut min_x = INFINITY;
    let mut max_x = NEG_INFINITY;
    let mut min_y = INFINITY;
    let mut max_y = NEG_INFINITY;

    let re = Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>").unwrap();

    let mut points: Vec<Point> = Vec::new();

    for line in lines {
        println!("Line: {}", line);
        let a = re.captures(line).unwrap();

        let x = a[1].parse::<f32>().unwrap();
        let y = a[2].parse::<f32>().unwrap();
        let vx = a[3].parse::<f32>().unwrap();
        let vy = a[4].parse::<f32>().unwrap();

        min_x = min(x, min_x);
        min_y = min(y, min_y);
        max_x = max(x, max_x);
        max_y = max(y, max_y);

        points.push(Point {
            x,
            y,
            vx,
            vy,
        });
    }

    let max_size = 200.0;

    //println!("field - x: {} {} {} y: {} {} {}", min_x, max_x, size_x, min_y, max_y, size_y);

    for i in 0..10700 {
        println!("");
        println!("After {} second(s) size: {} {}", i, max_x - min_x, max_y - min_y);
        
        if i < 10695 && i > 10675  {
            let size_x = /*max_size + 1.0;*/ max_x - min_x + 1.0;
            let size_y = /*max_size + 1.0;*/ max_y - min_y + 1.0;
            let mut field = vec![vec!['.'; size_x as usize]; size_y as usize];

            for point in &points {
                let index_x = (point.x - min_x);// / max_x * max_size;
                let index_y = (point.y - min_y);// / max_y * max_size;
                if index_x < 0.0 || index_y < 0.0 || index_x > size_x || index_y > size_y {
                    continue;
                }
                field[index_y as usize][index_x as usize] = '#';
            }
            print_field(field);
        }

        for mut point in &mut points {
            point.x += point.vx;
            point.y += point.vy;
        }

        min_x = INFINITY;
        max_x = NEG_INFINITY;
        min_y = INFINITY;
        max_y = NEG_INFINITY;
        for point in &points {
            min_x = min(point.x, min_x);
            min_y = min(point.y, min_y);
            max_x = max(point.x, max_x);
            max_y = max(point.y, max_y);
        }
    }
}
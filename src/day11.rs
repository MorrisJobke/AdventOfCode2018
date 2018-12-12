extern crate term;

use std::i32::MIN;
use std::time::Instant;

fn calculate(serial: i32, x: i32, y: i32) -> i32 {

    let rack_id = x + 10;
    let power = (rack_id * y + serial) * rack_id;
    let without_tail = (power - (power % 100)) / 100;

    return (without_tail % 10) - 5;
}

fn main() {
    // println!("{}", calculate(8, 3, 5));
    // println!("{}", calculate(57, 122, 79));
    // println!("{}", calculate(39, 217, 196));
    // println!("{}", calculate(71, 101, 153));

    let serial = 4172;
    let size = 300;

    let mut field = vec![vec![0; size as usize]; size as usize];

    let now = Instant::now();

    print!("Calculate the base grid … ");
    for y in 1..(size + 1) {
        for x in 1..(size + 1) {
            field[y - 1][x - 1] = calculate(serial, x as i32, y as i32);
        }
    }
    println!("finished in {}.{} seconds.", now.elapsed().as_secs(), now.elapsed().subsec_millis());

    /*
    Debug output of the field
    println!("");
    println!("Before:");
    println!("");
    let mut terminal = term::stdout().unwrap();
    for y in 0..size {
        if (y + 1) % 10 == 0 {
            terminal.attr(term::Attr::Bold).unwrap();
        }
        for x in 0..size {
            if (x + 1) % 10 == 0 {
                terminal.attr(term::Attr::Bold).unwrap();
            }
            if field[y][x] > 0 {
                terminal.fg(term::color::GREEN).unwrap();
            }
            if field[y][x] < 0 {
                terminal.fg(term::color::RED).unwrap();
            }
            print!("{:4}", field[y][x]);
            terminal.fg(term::color::BLACK).unwrap();

            if (x + 1) % 10 == 0 && (y + 1) % 10 != 0 {
                terminal.reset().unwrap();
            }
        }
        terminal.reset().unwrap();
        println!("");
    }
    println!("");
    */

    print!("Creating sums … ");
    let now = Instant::now();
    for y in 0..size {
        for x in 0..size {
            let mut sum = field[y][x];
            if x > 0 {
                sum = sum + field[y][x - 1];
            }
            if y > 0 {
                sum = sum + field[y - 1][x];
            }
            if x > 0 && y > 0 {
                sum = sum - field[y - 1][x - 1];
            }
            field[y][x] = sum;
        }
    }
    println!("finished in {}.{} seconds.", now.elapsed().as_secs(), now.elapsed().subsec_millis());

    println!("Searching for maximum area … ");
    let mut max_total = MIN;
    let mut max_position_x = 0;
    let mut max_position_y = 0;
    let mut max_total_size = 0;
    let now = Instant::now();

    for square_size in 1..(size + 1) {
        print!("Checking {} …", square_size);
        for y in 0..(size + 1 - square_size) {
            for x in 0..(size + 1 - square_size) {

                /* old way - naive implementation
                let mut total = 0;
                for i in 0..square_size {
                    for j in 0..square_size {
                        total = total + field[y + j][x + i];
                    }
                }
                */

                /* new way  - Summed-area table */
                let mut a = 0;
                if y > 0 && x > 0 {
                    a = field[y - 1                  ][x - 1                  ];
                }
                let mut b = 0;
                if y > 0 {
                    b = field[y - 1                  ][x     + square_size - 1];
                }
                let mut c = 0;
                if x > 0 {
                    c = field[y     + square_size - 1][x - 1                  ];
                }
                let d = field[y     + square_size - 1][x     + square_size - 1];

                let mut total = a - b - c + d;

                if square_size == 1 {
                    total = field[y][x];
                }
                /* end of new way */

                if total > max_total {
                    max_total = total;
                    max_position_x = x + 1;
                    max_position_y = y + 1;
                    max_total_size = square_size;
                }
            }
        }
        println!("(local) Position: {} {}, Total: {}, Size: {}, Time: {}.{} seconds", max_position_x, max_position_y, max_total, max_total_size, now.elapsed().as_secs(), now.elapsed().subsec_millis());
    }

    println!("Position: {} {}, Total: {}, Size: {}, Time: {}.{} seconds", max_position_x, max_position_y, max_total, max_total_size, now.elapsed().as_secs(), now.elapsed().subsec_millis());

}

use std::i32::MIN;

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

    for y in 1..(size + 1) {
        for x in 1..(size + 1) {
            field[y - 1][x - 1] = calculate(serial, x as i32, y as i32);
        }
    }

    let mut max_total = MIN;
    let mut max_position_x = 0;
    let mut max_position_y = 0;
    let mut max_total_size = 0;

    for square_size in 1..(size + 1) {
        println!("Checking {}…", square_size);
        for y in 1..(size + 2 - square_size) {
            for x in 1..(size + 2 - square_size) {

                let mut total = 0;
                for i in 0..square_size {
                    for j in 0..square_size {
                        total = total + field[y - 1 + j][x - 1 + i];
                    }
                }
                if total > max_total {
                    max_total = total;
                    max_position_x = x;
                    max_position_y = y;
                    max_total_size = square_size;
                }
            }
        }
        println!("(local) Position: {} {}, Total: {}, Size: {}", max_position_x, max_position_y, max_total, max_total_size);
    }

    println!("Position: {} {}, Total: {}, Size: {}", max_position_x, max_position_y, max_total, max_total_size);
}
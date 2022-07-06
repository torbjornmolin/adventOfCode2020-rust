use std::fs;
// use ndarray::Array4;
use ndarray::Array4;
const CUBE_SIDE: usize = SEED_SIDE*4;
const SEED_SIDE: usize = 8;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    //let content = fs::read_to_string("test.txt").unwrap();

    second(&content);
}

fn second(content: &str) {
    let mut current_board = get_start_cube(content);
    println!("Active cubes: {}", count_active(&current_board));
    for n in 1..7 {
        current_board = run_simulation_step(&current_board);
        println!("Iteration {}: Active cubes: {}",n, count_active(&current_board));
    }
}

fn run_simulation_step(board: &Array4::<u8>) -> Array4::<u8> {
    let mut result = Array4::<u8>::zeros((CUBE_SIDE, CUBE_SIDE, CUBE_SIDE, CUBE_SIDE));
    for w in 0..CUBE_SIDE  {
    for z in 0..CUBE_SIDE  {
        for x in 0..CUBE_SIDE {
            for y in 0..CUBE_SIDE  {
                let active_neighbours = active_neighbour_count(x as i32, y as i32, z as i32, w as i32, board);
                // cube is active
                if board[[x,y,z, w]] == 1 {
                    if is_on_edge(x,y,z, w) {
                        panic!("Active cube on edge of board!");
                    }
                    if active_neighbours == 2 || active_neighbours == 3 {
                        result[[x,y,z, w]] = 1;
                    }
                    else {
                        result[[x,y,z,w]] = 0;
                    }
                }
                // non-active
                else {
                    if active_neighbours == 3 {
                        // gets active
                        result[[x,y,z, w]] = 1;
                    }
                }
            }
        }
    }
    }

    result
}

fn is_on_edge(x: usize, y: usize, z: usize, w: usize) -> bool {
    if x == 0 || y == 0 || z == 0 || w == 0 {
        return true;
    }
    let max_coord = CUBE_SIDE - 1;
    if x == max_coord || y == max_coord || z == max_coord || w == max_coord {
        return true;
    }

    false
}

fn count_active(board: &Array4::<u8>) -> u32 {
    let mut result = 0;
    for w in 0..CUBE_SIDE {
    for x in 0..CUBE_SIDE {
        for y in 0..CUBE_SIDE {
            for z in 0..CUBE_SIDE {
                if board[[x,y,z, w]] == 1 {
                    result += 1;
                }
            }
        }
    }
}
    result
}

fn get_start_cube(content: &str) -> Array4::<u8> {
    let mut board = Array4::<u8>::zeros((CUBE_SIDE, CUBE_SIDE, CUBE_SIDE, CUBE_SIDE));
    let z_index = CUBE_SIDE/2;
    let w_index = CUBE_SIDE/2;
    let mut x_index;
    let mut y_index =  CUBE_SIDE/2 - SEED_SIDE;
    println!("Loading input to layer at z = {}, w = {}", z_index, w_index);
    for line in content.lines() {
        x_index =  CUBE_SIDE/2 - SEED_SIDE;
        for c in line.chars() {
            board[[x_index,y_index,z_index, w_index]] = match c {
                '#' => {
                    1
                }
                _ => { 
                    0
                }
            };
            x_index += 1;
        }
        y_index += 1;
    }


    board
}

fn get_value(x: i32, y: i32, z: i32,w: i32, board: &Array4<u8>) -> u8 {
    let max_value = (CUBE_SIDE -1) as i32;
    if x > max_value || x < 0 {
        return 0;
    }
    if y > max_value || y < 0 {
        return 0;
    }
    if z > max_value || z < 0 {
        return 0;
    }

    if w > max_value || w < 0 {
        return 0;
    }

    board[[x as usize,y as usize ,z as usize, w as usize]]
}

fn active_neighbour_count(x: i32, y: i32, z: i32,w: i32, board: &Array4<u8>) -> u8 {
    let mut result = 0;

    for z_offset in -1..2 {
        for w_offset in -1..2 {
        for y_offset in -1..2 {
            for x_offset in -1..2 {
                // skip self.
                if x_offset == 0 && y_offset == 0 && z_offset == 0 && w_offset == 0 {
                    continue;
                }
                // println!("xoffset {}, yoffset {}, zoffset {}", x_offset, y_offset, z_offset);
                let x_coord = x + x_offset;
                let y_coord = y + y_offset;
                let z_coord = z + z_offset;
                let w_coord = w + w_offset;

                if get_value(x_coord, y_coord, z_coord,w_coord, board) == 1 {
                    result += 1;
                }
            }
        }
    }
}

    result
}
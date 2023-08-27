use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn get_data() -> BufReader<File> {
    let file = File::open("./data/treetop_tree_house_input.txt").expect("Could not read the file.");
    BufReader::new(file)
}


fn generate_grid() -> Vec<Vec<u32>> {
    let buf_reader  = get_data();

    let mut grid = vec![];
    for line in buf_reader.lines() {
        let line = line.unwrap();
        let temp: Vec<u32> = line
            .chars()
            .filter_map(|e| e.to_digit(10))
            .collect();
        grid.push(temp);
    }

    grid
}


pub fn main_part01() {
    let grid = generate_grid();
    let mut trees_visible = grid.len() * 2 + (grid.len() - 2) * 2;

    for (i, row) in grid.iter().enumerate() {
        if i == 0 || i == grid.len() - 1 {
            continue;
        }

        for (k, height) in row.iter().enumerate() {
            if k == 0 || k == grid.len() - 1 {
                continue;
            }

            // left row
            let mut left_visible = true;
            for tree_height in row.iter().take(k ) {
                if tree_height >= height {
                    left_visible = false;
                    break
                }
            }


            // right row
            let mut right_visible = true;
            for tree_height in row.iter().skip(k + 1) {
               if tree_height >= height {
                   right_visible = false;
                   break
               }
            }

            // top columns
            let mut top_visible = true;
            for crow in grid.iter().take(i ) {
                if crow[k] >= *height {
                    top_visible = false;
                    break
                }
            }

            // bottom columns
            let mut bottom_visible = true;
            for crow in grid.iter().skip(i + 1) {
                if crow[k] >= * height {
                    bottom_visible = false;
                    break;
                }
            }

            if left_visible || right_visible || top_visible || bottom_visible {
                trees_visible += 1;
            }
        }
    }

    println!("{}", trees_visible);
}


pub fn main_part02() {
    let grid = generate_grid();
    let mut scenic_score = 0;

    for (i, row) in grid.iter().enumerate() {
        if i == 0 || i == grid.len() - 1 {
            continue;
        }

        for (k, height) in row.iter().enumerate() {
            if k == 0 || k == grid.len() - 1 {
                continue;
            }

            // left row
            let mut scenic_left = 0;
            for tree_height in row.iter().take(k ).rev() {
                scenic_left += 1;
                if tree_height >= height {
                    break;
                }
            }

            // right row
            let mut scenic_right = 0;
            for tree_height in row.iter().skip(k + 1) {
                scenic_right += 1;
                if tree_height >= height {
                    break;
                }
            }

            // top columns
            let mut scenic_top = 0;
            for crow in grid.iter().take(i ).rev() {
                scenic_top += 1;
                if crow[k] >= *height {
                    break;
                }
            }

            // bottom columns
            let mut scenic_bottom = 0;
            for crow in grid.iter().skip(i + 1) {
                scenic_bottom += 1;
                if crow[k] >= *height {
                    break;
                }
            }


            let tree_scenic = scenic_left * scenic_right * scenic_top * scenic_bottom;

            if tree_scenic > scenic_score {
                scenic_score = tree_scenic
            }
        }
    }

    println!("Best scenic score: {}", scenic_score);
}
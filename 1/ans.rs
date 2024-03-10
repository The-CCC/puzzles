/*
SAMPLE GRID
    1   4  8 19 2
    5  12  6  3 7
    18 20 14 17 22
    16 10 15 21 9
    11 24 23 13 12
Numbers will be randomized, return the smallest number chain.
 */
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::collections::HashSet; // Using HashSet for visited nodes

fn main() {
    let grid = vec![
        vec![1, 4, 8, 19, 2],
        vec![5, 12, 6, 3, 7],
        vec![18, 20, 14, 17, 22],
        vec![16, 10, 15, 21, 9],
        vec![11, 24, 23, 13, 12],
    ];

    println!("Grid: {:?}", grid);
    let (smallest_path, sum) = find_smallest_path(grid);
    println!("Smallest path: {:?}", smallest_path);
    println!("Sum: {sum}");
}

fn find_smallest_path(grid: Vec<Vec<i32>>) -> (Vec<i32>, i32) {
    let mut visited = HashSet::new();
    let mut stack = vec![(0, 0, vec![grid[0][0]], grid[0][0])];

    let mut smallest_path = vec![];
    let mut smallest_sum = i32::MAX;

    while let Some((x, y, path, sum)) = stack.pop() {
	// if there's only 1 element left (ie at bottom right) it's the only remaining "route" and only its distance is to be added
        if x == grid.len() - 1 && y == grid[0].len() - 1 && sum < smallest_sum {
            smallest_sum = sum;
            smallest_path = path;
        }
	else {
            visited.insert((x, y)); // if it's poss to insert an element, do so
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
		// all possible ways to move
                let new_x = x as i32 + dx;
	        let new_y = y as i32 + dy;
		// check if these new coordinates are:
		//    - within the grid's boundaries
		//    - aren't already visited
                if (0..grid.len() as i32).contains(&new_x) && (0..grid[0].len() as i32).contains(&new_y) && !visited.contains(&(new_x as usize, new_y as usize)) {
                    let mut new_path = path.clone(); // clone path (so as to not modify existing path for other tries)
                    new_path.push(grid[new_x as usize][new_y as usize]); // usize reqd for typecasting
                    stack.push((new_x as usize, new_y as usize, new_path, sum + grid[new_x as usize][new_y as usize]));
                }
            }
        }
    }
    (smallest_path, smallest_sum)
}

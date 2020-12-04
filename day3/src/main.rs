use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let map = fs::read_to_string("./input").expect("Couldn't open input");
    let slope = (3,1);
    println!("part1: hit {} trees", hit_trees(&map as &str, slope));
}

fn part2() {
    let map = fs::read_to_string("./input").expect("Couldn't open input");
    let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];
    for slope in slopes.iter() {
        println!("part2: hit {} trees", hit_trees(&map as &str, *slope));
    }

}

fn hit_trees(map: &str, slope: (i32, i32)) -> i32 {
    let mut trees = 0;
    let rows = map.lines().collect::<Vec<&str>>();
    let height = rows.len() as i32;
    let width = rows[0].trim().len() as i32;
    let mut sled_pos = (0,0);
    loop {
        if sled_pos.0 + slope.0 >= width {
            sled_pos.0 = sled_pos.0 + slope.0 - width;
        } else {
            sled_pos.0 = sled_pos.0 + slope.0;
        }
        sled_pos = (sled_pos.0,
                    sled_pos.1 + slope.1);
        if sled_pos.1 >= height {
            // off the bottom of the map
            break;
        }
        let line = rows[sled_pos.1 as usize].trim();

        if line.chars().collect::<Vec<char>>()[sled_pos.0 as usize] == '#' {
            trees += 1;
        }
    }
    trees
}

fn test() {
    let test_map = "..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#";
    
    println!("hit {} trees", hit_trees(test_map, (3,1)));
}
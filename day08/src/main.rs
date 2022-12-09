use std::fs;

fn main() {
    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";

    let content = fs::read_to_string(file_path).unwrap();

    let grid: Vec<Vec<usize>> = content.trim().lines()
        .map(|line| {
            line.chars().map(|digit| {
                digit.to_string().parse().unwrap()}
                ).collect()
        }).collect();

    part_1(&grid);
    part_2(&grid);
}

fn part_1(grid: &Vec<Vec<usize>>) {
    let mut visible_count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let current = grid[i][j];
            let vert: Vec<usize> = (0..grid.len()).fold(vec![], |mut vec, ii| {
                if grid[ii][j] >= current { vec.push(ii) };
                vec
            });
            let hori: Vec<usize> = (0..grid[i].len()).fold(vec![], |mut vec, jj| {
                if grid[i][jj] >= current { vec.push(jj) };
                vec
            });

            if j == hori[0] || j == hori[hori.len() - 1] ||
                i == vert[0] || i == vert[vert.len() - 1] {
                    visible_count += 1;
                }
        }
    }
    println!("{visible_count}");
}

fn part_2(grid: &Vec<Vec<usize>>) {
    let mut most_scenic = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let current = grid[i][j];

            let vert: Vec<Vec<usize>> = (0..grid.len()).fold(vec![], |mut vec, ii| {
                if grid[ii][j] >= current { vec.push(ii) };
                vec
            }).split(|&index| index == i)
            .map(|chunk| chunk.to_vec())
            .collect();

            let hori: Vec<Vec<usize>> = (0..grid[i].len()).fold(vec![], |mut vec, jj| {
                if grid[i][jj] >= current { vec.push(jj) };
                vec
            }).split(|&index| index == j)
            .map(|chunk| chunk.to_vec())
            .collect();

            let left = j - if hori[0].len() > 0 { hori[0][hori[0].len() - 1] } else { 0 };
            let right = if hori[1].len() > 0 { hori[1][0] } else { grid[i].len() - 1 } - j;

            let up = i - if vert[0].len() > 0 { vert[0][vert[0].len() - 1] } else { 0 };
            let down = if vert[1].len() > 0 { vert[1][0] } else { grid.len() - 1 } - i;

            let score = left * right * up * down;
            if score > most_scenic { most_scenic = score }
        }
    }

    println!("{most_scenic}");
}

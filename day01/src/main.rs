use std::fs;

fn main() {
    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";

    let elves = fs::read_to_string(file_path).unwrap()
        .trim().split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|item| {
                    item.parse::<usize>().unwrap()
                }).collect::<Vec<usize>>()
        }).collect::<Vec<Vec<usize>>>();

    part_1(&elves);
    part_2(&elves);
}

fn part_1(elves: &Vec<Vec<usize>>) {
    let mut max = 0;
    for elf in elves {
        let sum = elf.iter().sum();
        if sum > max { max = sum };
    }
    println!("{max}");
}

fn part_2(elves: &Vec<Vec<usize>>) {
    let mut elf_sums = elves.iter().map(|elf| {
        elf.iter().sum()
    }).collect::<Vec<usize>>();

    elf_sums.sort_by(|a, b| b.cmp(a));

    let mut max_three_elves = 0;
    for elf_dex in 0..3 {
        max_three_elves += elf_sums[elf_dex];
    }

    println!("{max_three_elves}");
}

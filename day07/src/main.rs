use std::fs;

fn size_files(content: &str) -> Vec<usize> {
    let mut dirs = Vec::new();

    fn recurse(lines: &mut dyn Iterator<Item = &str>, dirs: &mut Vec<usize>) -> usize {
        let mut size: usize = 0;

        loop {
            let next = lines.next().unwrap_or("$ cd ..");

            if next == "$ cd .." { break };

            if next.contains("$ cd") {
                size += recurse(lines, dirs);
            }

            let first_word = next.split(" ").collect::<Vec<&str>>()[0];

            if first_word.parse::<usize>().is_ok() {
                size += first_word.parse::<usize>().unwrap();
            }
        }

        dirs.push(size);

        size
    }
    recurse(&mut content.lines(), &mut dirs);

    dirs
}

fn part_1(directories: &Vec<usize>) {
    let sum = directories.iter()
        .fold(0, |sum, val| {
            if val <= &100000 { sum + val } else { sum }
        });

    println!("{sum}");
}

fn part_2(directories: &mut Vec<usize>) {
    directories.sort();
    let space_needed = 30000000 - (70000000 - directories[directories.len() - 1]);

    let greater: usize = directories.split(|size| size < &space_needed)
        .filter(|chunk| chunk.len() > 0)
        .collect::<Vec<&[usize]>>()[0].to_vec()[0];

    println!("{greater}");
}

fn main() {
    let file_path = "./src/input.txt";
    //let file_path = "./src/example.txt";
    let content = fs::read_to_string(file_path).unwrap();

    let mut directories = size_files(&content);

    part_1(&directories);
    part_2(&mut directories);
}

fn main() {
    let file = include_str!("./input1.txt");

    let parsed_lines = elf_calorie_iterator(file);

    println!("Part 1: {:?}", parsed_lines.first().unwrap());

    println!("Part 2: {:?}", parsed_lines.iter().take(3).sum::<i32>());
}

fn elf_calorie_iterator(input: &str) -> Vec<i32> {
    let lines = input.split("\n\n");

    let mut parsed_lines: Vec<i32> = lines
        .map(|line| {
            line.split("\n")
                .flat_map(|single_calory| single_calory.parse::<i32>())
                .sum()
        })
        .collect();

    parsed_lines.sort_by(|a, b| b.cmp(a));
    return parsed_lines;
}

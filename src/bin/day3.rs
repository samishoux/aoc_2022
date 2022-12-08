use std::collections::HashMap;

fn generate_priorities() -> HashMap<char, usize> {
    let mut hash_map = HashMap::new();
    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    for (i, letter) in alphabet.char_indices() {
        hash_map.insert(letter, i + 1);
    }

    for (i, letter) in alphabet.to_uppercase().char_indices() {
        hash_map.insert(letter, i + 1 + 26);
    }

    return hash_map;
}

fn resolve_part_1(input: &str) -> usize {
    let hash_map = generate_priorities();

    let inputs = input.split("\n").map(|word| word.split_at(word.len() / 2));

    let mut sum: usize = 0;
    for (first, second) in inputs {
        let mut first_seen_words: HashMap<char, usize> = HashMap::new();
        let mut second_seen_words: HashMap<char, usize> = HashMap::new();
        for n in 0..first.len() {
            let first_char = first.chars().nth(n).unwrap();
            let second_char = second.chars().nth(n).unwrap();
            first_seen_words.insert(first_char, 1);
            second_seen_words.insert(second_char, 1);

            if first_seen_words.contains_key(&second_char) {
                if second_seen_words.get(&second_char).is_some()
                    && *second_seen_words.get(&second_char).unwrap() == 1
                {
                    *first_seen_words.get_mut(&second_char).unwrap() += 1;
                }
                // first_seen_words[second_char]
            }

            if second_seen_words.contains_key(&first_char) {
                if first_seen_words.get(&first_char).is_some()
                    && *first_seen_words.get(&first_char).unwrap() == 1
                {
                    *second_seen_words.get_mut(&first_char).unwrap() += 1;
                }
                // first_seen_words[second_char]
            }
        }

        for (letter, count) in first_seen_words {
            if count <= 1 {
                continue;
            }

            sum += hash_map.get(&letter).unwrap()
        }

        for (letter, count) in second_seen_words {
            if count <= 1 {
                continue;
            }

            sum += hash_map.get(&letter).unwrap()
        }
    }
    return sum;
}

fn resolve_part_2(input: &str) -> usize {
    let hash_map = generate_priorities();

    let inputs = input
        .split("\n")
        .map(|word| word.char_indices().collect())
        .collect::<Vec<Vec<(usize, char)>>>();

    for word in inputs {
        for letter in word {
            //check if letter exist in combination of 3
            //for the past / next 3 ?
        }
        // println!("{:?}", char);
    }

    let mut sum: usize = 0;

    return sum;
}

fn main() {
    let input = include_str!("./input3.prod");

    let sum = resolve_part_1(input);

    println!("Part 1: {:?}", sum);
    let input = include_str!("./input3.dev");

    let sum = resolve_part_2(input);
    println!("Part 2: {:?}", sum);
    // print!("{:?}", hash_map)
    //take input
    //split on each lines
    //split at the middle point in the string
    //check for duplicate in both outputs

    //Lowercase item types a through z have priorities 1 through 26.
    //Uppercase item types A through Z have priorities 27 through 52.
    //sum all the priorities
}

use anyhow::Error;
use std::str::FromStr;

struct Dictionary {
    name: String,
    files: Vec<File>,
    dictionaries: Vec<Dictionary>,
}

impl Default for Dictionary {
    fn default() -> Self {
        Self {
            name: Default::default(),
            files: Default::default(),
            dictionaries: Default::default(),
        }
    }
}

impl Dictionary {
    fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

impl FromStr for Dictionary {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //take this and convert into a dictionary
        //dir d
        if !s.starts_with("dir") {
            return Err(Error::msg(
                "Unexpected string format. Expected to have 'dir' at the start of the string",
            ));
        }
        let Some((_, name)) = s.split_once(" ") else  {
            return Err(Error::msg("Could not split on space for the dictonary"));
        };

        return Ok(Dictionary::new(name));

        todo!()
    }
}

struct File {
    name: String,
    size: usize,
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //take this and convert into a file
        //14848514 b.txt
        let Some((size, name)) = s.split_once(" ") else {
            return Err(Error::msg("Could not split on space for the file"));
        };

        let Ok(size) = size.parse::<usize>() else {
            return Err(Error::msg("Could not parse file size into usize"));
        };

        return Ok(File {
            name: name.into(),
            size,
        });
    }
}

#[test]
fn test_from_str_file() {
    let input = "14848514 b.txt";
    let file = input.parse::<File>();
    assert!(file.is_ok());
    let file = file.unwrap();
    assert!(file.name == "b.txt");
    assert!(file.size == 14848514);
}

fn main() {}

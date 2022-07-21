use std::{fs, io};
use std::collections::linked_list::LinkedList;

const INPUT_FILE_PATH: &str = "./in/input.txt";
const IS_READING_FROM_FILE: bool = false;

fn get_data_reader(is_reading_from_file: bool) -> DataReader {
    let data_source: DataSource = match is_reading_from_file {
        true => DataSource::File(String::from(INPUT_FILE_PATH)),
        false => DataSource::StdIo
    };
    DataReader::from_data_source(data_source)
}

fn main() {
    let mut data_reader = get_data_reader(IS_READING_FROM_FILE);
    let number_of_tests = data_reader.read_i32();

    for _ in 0..number_of_tests {
        let _ = data_reader.read_string();
        let game_results = data_reader.read_string();
        let mut letters: Vec<char> = game_results.chars().collect();
        letters.sort_by(|a, b| b.cmp(a));
        letters.dedup();
        let unique_letters = letters.into_iter().count() as i32;
        let all_letters = game_results.chars().count() as i32;
        println!("{}", unique_letters * 2 + all_letters - unique_letters)
    }
}

enum DataSource {
    StdIo,
    File(String)
}

struct DataReader {
    data_list: LinkedList<String>,
    data_source: DataSource
}

impl DataReader {
    fn from_data_source(data_source: DataSource) -> DataReader {
        DataReader {
            data_list: match data_source {
                DataSource::StdIo => LinkedList::new(),
                DataSource::File(ref file_name) =>
                    fs::read_to_string(String::from(file_name)).unwrap().split_whitespace().map(|x| x.to_owned()).collect()
            },
            data_source
        }
    }

    fn read_string(&mut self) -> String {
       match self.data_source {
           DataSource::StdIo => {
               let mut input = String::new();
               io::stdin().read_line(&mut input).unwrap();
               String::from(input.trim())
           },
           DataSource::File(_) => {
               String::from(self.data_list.pop_front().unwrap())
           }
       }
    }

    fn read_i32(&mut self) -> i32 {
        self.read_string().parse().unwrap()
    }
}

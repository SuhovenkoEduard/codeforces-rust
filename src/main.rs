use std::{fs, io};
use std::collections::linked_list::LinkedList;

const INPUT_FILE_PATH: &str = "./in/input.txt";
const IS_READING_FROM_FILE: bool = true;

fn main() {
    let mut data_reader = get_data_reader(IS_READING_FROM_FILE);
    let t = data_reader.next_i32();

    for _ in 0..t {
        let n = data_reader.next_i32();
        let a: Vec<i32> = (0..n).into_iter().map(|_| data_reader.next_i32()).collect();
        let p: Vec<i32> = a.iter().enumerate().map(|(i, x)| x.to_owned() - i as i32 - 1).collect();
        let mut vec = a.iter().enumerate()
            .filter(|(i, _)| p[*i] < 0)
            .map(|(_, x)| x.to_owned())
            .collect::<Vec<i32>>();
        vec.sort();
        let mut list: LinkedList<i32> = vec.into_iter().collect();
        let mut answer = 0;
        for i in 0..n {
            if p[i as usize] >= 0 { continue; }
            while !list.is_empty() && list.front().unwrap().to_owned() <= i + 1 {
                list.pop_front();
            }
            answer += list.len();
        }
        println!("{}", answer);
    }
}

// DataReader
fn get_data_reader(is_reading_from_file: bool) -> DataReader {
    let data_source: DataSource = match is_reading_from_file {
        true => DataSource::File(String::from(INPUT_FILE_PATH)),
        false => DataSource::StdIo
    };
    DataReader::from_data_source(data_source)
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
                DataSource::File(ref file_name) => {
                    fs::read_to_string(String::from(file_name)).unwrap()
                        .split_whitespace().map(str::to_string).collect()
                }
            },
            data_source
        }
    }

    fn next_string(&mut self) -> String {
       if self.data_list.is_empty() {
           self.read_string().split_whitespace().map(str::to_string).for_each(|s| self.data_list.push_back(s));
       }

       self.data_list.pop_front().unwrap()
    }

    fn next_i32(&mut self) -> i32 {
        self.next_string().parse().unwrap()
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
}
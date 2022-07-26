use std::{cmp, fs, io};
use std::collections::linked_list::LinkedList;

const INPUT_FILE_PATH: &str = "./in/input.txt";
const IS_READING_FROM_FILE: bool = true;

const MAX_POWER: usize = 64;

fn main() {
    let mut data_reader = get_data_reader(IS_READING_FROM_FILE);
    let t = data_reader.next_i32();

    for _ in 0..t {
        let n = data_reader.next_i32() as usize;
        let k = data_reader.next_i32() as i64;

        let a: Vec<i64> = (0..n).into_iter().map(|_| data_reader.next_i32() as i64).collect();
        let mut dp: Vec<Vec<i64>> = vec![vec![i64::MIN; MAX_POWER]; n + 1];
        dp[0][0] = 0;
        for i in 0..n {
            for j in 0..MAX_POWER {
                if dp[i][j] == i64::MIN { continue; }
                let divided = a[i].checked_shr(j as u32).unwrap_or(0);
                dp[i + 1][j] = cmp::max(dp[i][j] - k + divided, dp[i + 1][j]);

                let next = if j + 1 == MAX_POWER { j } else { j + 1 };
                let divided = a[i].checked_shr((j + 1) as u32).unwrap_or(0);
                dp[i + 1][next] = cmp::max(dp[i][j] + divided, dp[i + 1][next]);
            }
        }

        let mut answer = i64::MIN;
        for j in 0..MAX_POWER {
            answer = cmp::max(dp[n][j], answer);
        }
        println!("{answer}");
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
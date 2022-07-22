use std::{fs, io};
use std::collections::linked_list::LinkedList;

const INPUT_FILE_PATH: &str = "./in/input.txt";
const IS_READING_FROM_FILE: bool = false;

const ENGLISH_ALPHABET_SIZE: i32 = 26;

struct TreeNode {
    is_end: bool,
    next_map: Vec<usize>
}

impl TreeNode {
    fn new() -> TreeNode {
        TreeNode {
            next_map: vec![0; ENGLISH_ALPHABET_SIZE as usize],
            is_end: false
        }
    }
}

struct StringTree {
    nodes: Vec<TreeNode>
}

impl StringTree {
    fn new() -> StringTree {
        StringTree {
            nodes: vec![TreeNode::new()]
        }
    }

    fn get_char_index(ch: char) -> usize {
        (ch as i32 - 'a' as i32) as usize
    }

    fn add_string(&mut self, s: &String) {
        let mut current_node = 0;
        s.chars().for_each(|ch| {
            let char_index = StringTree::get_char_index(ch);
            let mut next_node = self.nodes[current_node].next_map[char_index];
            if next_node == 0 {
                next_node = self.add_node();
                self.nodes[current_node].next_map[char_index] = next_node;
            }
            current_node = next_node
        });
        self.nodes[current_node].is_end = true;
    }

    fn find_every_substring(&mut self, s: &String) -> Vec<bool> {
        let mut result = vec![false; s.len()];
        let mut current_node = 0;
        s.chars().enumerate().for_each(|(i, ch)| {
            let char_index = StringTree::get_char_index(ch);
            let next_index = self.nodes[current_node].next_map[char_index];
            result[i] = self.nodes[next_index].is_end;
            current_node = next_index
        });
        result
    }

    fn add_node(&mut self) -> usize {
        self.nodes.push(TreeNode::new());
        self.nodes.len() - 1
    }
}

fn main() {
    let mut data_reader = get_data_reader(IS_READING_FROM_FILE);
    let t = data_reader.next_i32();

    for _ in 0..t {
        let n = data_reader.next_i32();
        let a: Vec<String> = (0..n).into_iter().map(|_| data_reader.next_string()).collect();
        let mut left_tree = StringTree::new();
        let mut right_tree = StringTree::new();
        a.iter().for_each(|s| {
            left_tree.add_string(&s);
            right_tree.add_string(&s.chars().rev().collect());
        });
        let mut result = vec![false; a.len()];
        a.iter().enumerate().for_each(|(string_index, s)| {
            let left = left_tree.find_every_substring(&s);
            let right: Vec<bool> = right_tree.find_every_substring(&s.chars().rev().collect()).into_iter().rev().collect();
            for i in 0..s.len() - 1 {
                if left[i] && right[i + 1] {
                    result[string_index] = true;
                }
            }
        });
        println!("{}", format_bool_vec(&result));
    }
}

// formatters
fn format_vec<T: ToString>(vec: &Vec<T>) -> String {
    vec.iter().map(|x| x.to_string() + " ").collect::<Vec<String>>().join("")
}

fn format_bool_vec(vec: &Vec<bool>) -> String {
    vec.iter().map(|x| if x.to_owned() { 1 } else { 0 }).map(|x| x.to_string()).collect::<Vec<String>>().join("")
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

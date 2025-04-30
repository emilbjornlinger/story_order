use topological_sort::TopologicalSort;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut ts = TopologicalSort::<u32>::new();
    let mut mappings = HashMap::<u32, Vec<u32>>::new();

    let file = open_file("./src/input/test2.txt");

    let reader = BufReader::new(file);
    let lines = reader.lines();

    for (_, line) in lines.enumerate() {
        let line = line.unwrap();

        let split: Vec<_> = line.split('|').collect();
        let node: u32 = split[0].parse::<u32>().unwrap();
        let mut deps = Vec::<u32>::new();

        let mut funcs_to_change: Vec<_> = split[1].split(',').collect();
        funcs_to_change.retain(|x| *x != "" && *x != " ");

        for val in funcs_to_change.iter_mut() {
            *val = val.trim();
        }

        for val in funcs_to_change.iter() {
            let num = val.parse::<u32>().unwrap();
            ts.add_dependency(node, num);
            deps.push(num);
        }

        mappings.insert(node, deps);
    }

    let file = open_file("./src/input/mappings.txt");

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut file_map = HashMap::<u32, String>::new();

    for (_, line) in lines.enumerate() {
        let line = line.unwrap();

        let split: Vec<_> = line.split('-').collect();
        let node = split[0].trim();
        let node: u32 = node.parse::<u32>().unwrap();

        let func = split[1].trim();

        file_map.insert(node, func.to_string());
    }

    println!("Topological order:");
    let mut idx = 1;
    while let Some(i) = ts.pop() {
        println!("{i}:");
        println!("Function: {}", match file_map.get(&i) {Some(name) => name, None => "unknown function"});
        println!("----");
        let vec = Vec::<u32>::new();
        let vec = match mappings.get(&i) {Some(vec) => vec, None => &vec};
        for file_idx in vec.iter() {
            println!("{file_idx}: {}", match file_map.get(&file_idx) {Some(name) => name, None => "unknown function"});
        }
        println!("----\n");

        idx += 1;
    }
}

fn open_file(path_str: &str) -> File {
    let path = Path::new(path_str);
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    file
}

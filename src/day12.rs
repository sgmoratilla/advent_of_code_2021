use std::collections::HashMap;
use std::fs::File;
use std::{io, process};
use std::env::current_exe;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::iter::Map;

fn main() {
    let mut data = read_file("src/day12.txt");
    paths(&data, true);
}

fn paths(graph: &HashMap::<String, Vec<String>>, allow_one_single_repetition: bool) -> u32 {

    let mut path_stack = Vec::new();
    let mut completed_paths = Vec::new();

    find_caves(&"start".to_string(), graph, &mut path_stack, &mut completed_paths, allow_one_single_repetition);

    return completed_paths.len() as u32;
}

fn find_caves(current_node: &String, graph: &HashMap::<String, Vec<String>>, path_stack: &mut Vec<String>, completed_paths: &mut Vec<Vec<String>>, allow_one_single_repetition: bool) {
    if "end" == current_node {
        completed_paths.push(path_stack.clone());
        return;
    }

    for node in graph.get(current_node).unwrap() {
        if !should_visit(node, path_stack, allow_one_single_repetition) {
            continue;
        }

        path_stack.push(node.clone());
        find_caves(node, graph, path_stack, completed_paths, allow_one_single_repetition);
        path_stack.pop();
    }
}

fn should_visit(node: &String, path_stack: &Vec<String>, allow_one_single_repetition: bool) -> bool {

    if !is_lowercase(node) {
        return true;
    }

    // visiting again
    if !path_stack.contains(node) {
        return true;
    }

    if !allow_one_single_repetition {
        return false;
    }

    let (most_repeated, repetitions) = most_repeated(path_stack);
    if repetitions == 2 {
        return false;
    }

    return true;
}

fn most_repeated(path_stack: &Vec<String>) -> (String, usize) {
    let max = path_stack.iter()
        .filter(|&x| is_lowercase(x))
        .fold(HashMap::<String, usize>::new(), |mut m, x| {
            *m.entry(x.clone()).or_default() += 1;
            m
        })
        .into_iter()
        .max_by_key(|(_, v)| *v);

    return if max.is_some() {  max.unwrap() } else { ("".to_string(), 0) };
}

fn read_file(path: &str) -> HashMap::<String, Vec<String>> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    return reader_to_data(&mut reader);
}

fn reader_to_data<R: io::Read>(reader: &mut BufReader<R>) -> HashMap::<String, Vec<String>> {
    let lines =  reader.lines().peekable();

    let mut graph =  HashMap::<String, Vec<String>>::new();

    for l in lines {
        let l = l.unwrap();

        let splits = l.split("-").map(|x| x.to_string()).collect::<Vec<String>>();
        let first = splits.get(0).unwrap();
        let second = splits.get(1).unwrap();

        link(&mut graph, first, second);
        link(&mut graph,  second, first);
    }

    return graph;
}

fn link(graph: &mut HashMap::<String, Vec<String>>, start: &String, end: &String) {
    if "end" == start {
        return;
    }

    if "start" == end {
        return;
    }

    if graph.contains_key(start) {
        let mut edges = graph.get_mut(start).unwrap();
        edges.push(end.to_string());
    } else {
        let mut edges = Vec::<String>::new();
        edges.push(end.to_string());
        graph.insert(start.to_string(), edges);
    }
}


fn is_lowercase(string: &String) -> bool {
    return string == &(string.to_lowercase());
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader};
    use crate::{paths, read_file, reader_to_data};

    #[test]
    fn example1() {
        let data =
"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        let mut reader = BufReader::new(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let paths = paths(&data, false);
        assert_eq!(paths, 10);
    }

    #[test]
    fn day12a() {
        let data = read_file("src/day12.txt");

        let paths = paths(&data, false);
        assert_eq!(paths, 5076);
    }

    #[test]
    fn example21() {
        let data =
"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        let mut reader = BufReader::new(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let paths = paths(&data, true);
        assert_eq!(paths, 36);
    }


    #[test]
    fn example22() {
        let data =
"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

        let mut reader = BufReader::new(data.as_bytes());
        let data = reader_to_data(&mut reader);

        let paths = paths(&data, true);
        assert_eq!(paths, 103);
    }

    #[test]
    fn day12b() {
        let data = read_file("src/day12.txt");

        let paths = paths(&data, true);
        assert_eq!(paths, 145643);
    }
}

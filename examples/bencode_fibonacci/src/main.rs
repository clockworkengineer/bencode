use bencode_lib::{FileDestination, FileSource, Node, parse, stringify};
use std::path::Path;
fn read_sequence(path: &Path) -> Result<Vec<i64>, String> {
    if !path.exists() {
        return Ok(vec![1, 1]);
    }
    match  FileSource::new(path.to_str().unwrap()) {
        Ok(mut file) => match parse(&mut file) {
            Ok(Node::List(items)) => {
                items.into_iter()
                .map(|n| match n {
                    Node::Integer(i) => Ok(i),
                    _ => Err("Invalid sequence format".to_string())
                })
                .collect()
        }
        _ => Err("Invalid file format".to_string())
        },
        Err(e) => Err(format!("Failed to open file: {}", e))
    }
}

fn calculate_next(sequence: &[i64]) -> i64 {
    let len = sequence.len();
    sequence[len - 1] + sequence[len - 2]
}

fn write_sequence(path: &Path, sequence: &[i64]) -> Result<(), String> {
    let list = Node::List(sequence.iter().map(|&n| Node::Integer(n)).collect());
    let  file = FileDestination::new(path.to_str().unwrap_or(""));
    match file {
        Ok(mut f) => { stringify(&list, &mut f); Ok(()) }
        Err(e) => { Err( e.to_string())}
    }
}

fn main() {
    let path = Path::new("fibonacci.bencode");

    match read_sequence(path) {
        Ok(mut sequence) => {
            let next = calculate_next(&sequence);
            sequence.push(next);

            if let Err(e) = write_sequence(path, &sequence) {
                eprintln!("Failed to write sequence: {}", e);
                return;
            }

            println!("Added Fibonacci number: {}", next);
        }
        Err(e) => eprintln!("Failed to read sequence: {}", e)
    }
}

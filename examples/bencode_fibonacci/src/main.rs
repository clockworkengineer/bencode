use bencode_lib::{FileDestination, FileSource, Node, parse, stringify};
use std::path::Path;
fn read_sequence(path: &Path) -> Result<Node, String> {
    if !path.exists() {
        return Ok(Node::List([Node::Integer(1), Node::Integer(1)].into()));
    }

    match FileSource::new(&path.to_string_lossy()) {
        Ok(mut file) => match parse(&mut file) {
            Ok(Node::List(list)) => Ok(Node::List(list)),
            Ok(_) => Err("Invalid file format: expected a list".to_string()),
            Err(e) => Err(e),
        },
        Err(e) => Err(format!("Failed to open file: {}", e)),
    }

}

fn add_next(sequence: &mut Node) {
    if let Node::List(items) = sequence {
        if items.len() < 2 {
            return;
        }
        match (&items[items.len() - 2], &items[items.len() - 1]) {
            (Node::Integer(a), Node::Integer(b)) => {
                if let Some(sum) = a.checked_add(*b) {
                    items.push(Node::Integer(sum));
                }
            }
            _ => {}
        }
    }

}

fn write_sequence(path: &Path, sequence: &Node) -> Result<(), String> {
    let  file = FileDestination::new(path.to_str().unwrap_or(""));
    match file {
        Ok(mut f) => { stringify(&sequence, &mut f); Ok(()) }
        Err(e) => { Err( e.to_string())}
    }
}

fn main() {
    let path = Path::new("fibonacci.bencode");
    match read_sequence(path) {
        Ok(mut sequence) => {
            add_next(&mut sequence);
            if let Err(e) = write_sequence(path, &sequence) {
                eprintln!("Failed to write sequence: {}", e);
                return;
            }
        }
        Err(e) => eprintln!("Failed to read sequence: {}", e)
    }
}

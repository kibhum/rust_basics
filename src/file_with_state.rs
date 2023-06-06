use std::fmt;
use std::fmt::Display;
#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    // To implement
    // std::fmt::Display, a
    // single fmt method
    // must be defined for
    // your type.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}
impl Display for File {
    // To implement
    // std::fmt::Display, a
    // single fmt method
    // must be defined for
    // your type.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // It is common to defer to the inner
        // types’ Display implementation via
        // the write! macro.
        write!(f, "<{} ({})>", self.name, self.state)
    }
}
impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        // In this code, read() never fails, but
        // we still wrap read_length in Ok
        // because we’re returning Result.
        Ok(read_length)
    }
}

// First appearance of Result<T, E>,
// where T is an integer of type usize
// and E is a String. Using String
// provides arbitrary error messages.

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

pub fn body() {
    let mut f5 = File::new("5.txt");
    let mut buffer: Vec<u8> = vec![];

    if f5.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f5 = open(f5).unwrap();
    let f5_length = f5.read(&mut buffer).unwrap();
    f5 = close(f5).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f5);
    println!("{} is {} bytes long", &f5.name, f5_length);
    println!("{}", text);
}

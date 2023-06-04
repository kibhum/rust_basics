// Allows println! to print File. The std::fmt::Debug
// trait works in conjunction with {:?} within the
// macro to enable File as a printable string.
#[derive(Debug)]
struct File {
    name: String,
    // Using Vec<u8>, provides access to some useful
    // conveniences like dynamic sizing, which makes it
    // possible to simulate writing to a file
    data: Vec<u8>,
}

pub fn body() {
    let f1 = File {
        // String::from generates
        // owned strings from string
        // literals, which are slices.
        name: String::from("f1.txt"),
        data: Vec::new(),
    };
    // Accessing fields uses the . operator.
    // Accessing fields by reference prevents
    // their use after move issues.
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}

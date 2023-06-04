// Relaxes compiler warnings
// while working through ideas
#![allow(unused_variables)]

// Creates a type alias. The compiler
// won’t distinguish between String &
// File, but your source code will.
type File = String;
// A global variable, static mut (or mutable static), with a static lifetime
// that’s valid for the life of the program
static mut ERROR: i32 = 0;

fn open(f: &mut File) -> bool {
    true
}
fn close(f: &mut File) -> bool {
    true
}
// Relaxes a compiler warning
// about an unused function
#[allow(dead_code)]
// Returns the number
// of bytes read
fn read(f: &mut File, save_to: &mut Vec<u8>) -> usize {
    // Makes a copy of the data here
    // because save_to.append()
    // shrinks the input Vec<T>
    let mut tmp = f.data.clone();
    let read_length = tmp.len();
    // Ensures that there is
    // sufficient space to fit
    // the incoming data
    save_to.reserve(read_length);
    // Allocates sufficient data in the save_to
    // buffer to hold the contents of f
    save_to.append(&mut tmp);
    read_length
}

fn run() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];
    // Does the hard work of
    // interacting with the file
    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);
    // Converts Vec<u8> to
    // String. Any bytes that
    // are not valid UTF-8 are
    // replaced with 􀳦.
    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    // Views the bytes 114, 117, 115,
    // 116, and 33 as an actual word
    println!("{}", text);

    let mut f = File::new("something.txt");
    read(f, buffer);

    // Accessing and
    // modifying static
    // mut variables
    // requires the use of
    // an unsafe block.
    // This is Rust’s way
    // of disclaiming all
    // responsibility.
    unsafe {
        // Checks
        // the ERROR
        // value. Error
        // checking
        // relies on the
        // convention
        // that 0 means
        // no error.
        if ERROR != 0 {
            panic!("An error has occurred while reading the file ")
        }
        close(f);
        unsafe {
            if ERROR != 0 {
                panic!("An error has occurred while closing the file ")
            }
        }
    }
}

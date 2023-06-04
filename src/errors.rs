use rand::random;

static mut ERROR: isize = 0;
// Creates a zero-sized type to stand in for
// a struct while we’re experimenting
struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    // Returns true one out
    // of eight times this
    // function is called
    if random() && random() && random() {
        unsafe {
            // Sets ERROR to 1, notifying
            // the rest of the system that
            // an error has occurred
            ERROR = 1;
        }
    }
    // Always reads 0 bytes
    0
}
// Keeping buffer mutable for
// consistency with other code even
// though it isn’t touched here
#[allow(unused_mut)]
fn rand_body() {
    let mut f = File;
    let mut buffer = vec![];
    read(&f, &mut buffer);

    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}

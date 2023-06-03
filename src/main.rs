// Importing functions from lib.rs
use rust_in_action_practice::creating_grep_lite::creating_grep_lite;
// Import second way
mod arrays_file;
fn main() {
    creating_grep_lite();
    arrays_file::arrays_mod::arrays();
    arrays_file::arrays_mod::arr_iteration();
}

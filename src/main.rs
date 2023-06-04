// Importing functions from lib.rs
use rust_in_action_practice::creating_grep_lite::creating_grep_lite;
// Import second way
mod arrays_file;
mod clap_with_regex;
mod reading_from_files;
mod regex;
mod vectors_file;

fn main() {
    // creating_grep_lite();
    // arrays_file::arrays_mod::arrays();
    // arrays_file::arrays_mod::arr_iteration();
    // vectors_file::vectors();
    // regex::regex();
    // clap_with_regex::clap_regex();
    // reading_from_files::file_read_manually();
    // reading_from_files::file_read_buf_reader();
    reading_from_files::read_lines_from_file();
}

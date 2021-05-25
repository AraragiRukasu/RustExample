use super::common_printer::Printer;
use std::fs;

const OUTPUT_FILENAME: &str = "output.txt";

pub struct FilePrinter {}

impl Printer for FilePrinter {
    fn print(&self, message: &String) {
        if let Err(e) = fs::write(OUTPUT_FILENAME, message) {
            println!("Error ocurred printing results, {:?}", e);
        } else {
            println!("Array is too large to be printed in cosole, we printed it on {}", OUTPUT_FILENAME);
        }
    }
}
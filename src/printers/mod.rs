mod common_printer;
mod file_printer;
mod standard_printer;
mod default_printer;

use common_printer::Printer;
use default_printer::DefaultPrinter;
use file_printer::FilePrinter;
use standard_printer::StandardPrinter;

const CONSOLE_MAX_OUTPUT:usize = 500;
const FILE_MAX_OUTPUT:usize = 1_000_000;

pub fn print_array(array: &[u32]) {
    let output_string = format!("Result: {:?}", array);
    let printer: Box<dyn Printer>;

    if array.len() <= CONSOLE_MAX_OUTPUT {
        printer = Box::new(StandardPrinter{});
    } else if array.len() <= FILE_MAX_OUTPUT {
        printer = Box::new(FilePrinter{});
    } else {
        printer = Box::new(DefaultPrinter{});
    }

    printer.print(&output_string);
}
mod common_printer;
mod file_printer;
mod standard_printer;

use common_printer::Printer;
use file_printer::FilePrinter;
use standard_printer::StandardPrinter;

const CONSOLE_MAX_OUTPUT:usize = 500;

pub fn print_array(array: &[u32]){
    let output_string = format!("Result: {:?}", array);
    let printer: Box<dyn Printer>;

    if array.len() > CONSOLE_MAX_OUTPUT {
        printer = Box::new(FilePrinter{});
    } else {
        printer = Box::new(StandardPrinter{});
    }

    printer.print(&output_string);
}
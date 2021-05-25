use super::common_printer::Printer;

pub struct StandardPrinter {}

impl Printer for StandardPrinter{
    fn print(&self, message: &String){
        println!("{}", message);
    }
}
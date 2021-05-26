pub trait Printer {
    fn print(&self, message: &String) {
        println!("This output could not be printed, message length(bytes): {}", message.len());
    }
}
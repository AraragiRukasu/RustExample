mod transformers;
mod printers;

use stopwatch::Stopwatch;
use transformers::array::rotate_array_left;
use printers::print_array;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let element_quantity = &args[1].parse::<u32>().unwrap();
    let positions = &args[2].parse::<usize>().unwrap(); 

    process(*element_quantity, *positions);
}

fn process(element_quantity: u32, positions: usize){
    let mut array:Vec<u32> = (0..element_quantity).collect();
    
    let mut sw = Stopwatch::start_new();
    rotate_array_left(&mut array, positions);
    sw.stop();

    println!("Elapsed: {:?}", sw.elapsed());

    print_array(&array);
}




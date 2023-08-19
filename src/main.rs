use std::io::{self, BufRead};

mod parse;
mod utils;

fn main() {
    let stations = parse::parse_file("matrix.txt");
    // let matrix = parse::create_adjacency_matrix(&stations, true);
    let mut conns: Vec<u8>;
    let mut x_location = String::new();
    loop {
        io::stdin().lock().read_line(&mut x_location).unwrap();
        println!("{:?}", x_location);
        let xloc: usize = x_location.trim().parse().unwrap();
        x_location = String::new();
        conns = stations
            .get(xloc - 1)
            .unwrap()
            .find_possible_from(&stations, &utils::get_tickets());
        conns.sort();
        conns.dedup();
        println!("{:?}", conns);
    }
}

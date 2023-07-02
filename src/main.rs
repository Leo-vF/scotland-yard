mod parse;
mod utils;

fn main() {
    let stations = parse::parse_file("matrix.txt");
    let matrix = parse::create_adjacency_matrix(&stations, true);
    let conns = &stations
        .get(0)
        .unwrap()
        .find_possible_from(&stations, vec![utils::Ticket::Taxi, utils::Ticket::Taxi]);
    println!("{:?}", conns.len());
    for con in conns {
        print!("{:?}, ", con);
    }
}

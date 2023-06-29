mod parse;

fn main() {
    let stations = parse::parse_file("matrix.txt");
    let matrix = parse::create_adjacency_matrix(stations, true);
    print!("{:?}", matrix);
}

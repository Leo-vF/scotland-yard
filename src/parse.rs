use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

pub struct Station {
    taxi: Vec<u8>,
    bus: Vec<u8>,
    subway: Vec<u8>,
    boat: Vec<u8>,
}
fn parse_conn(conn: &str) -> Vec<u8> {
    conn.split(",")
        .filter(|conn| !conn.is_empty())
        .map(|dest| dest.parse::<u8>().unwrap())
        .collect()
}
fn parse_line(line: &str) -> Station {
    let conns: Vec<Vec<u8>> = line.split(";").map(|conn| parse_conn(conn)).collect();

    return Station {
        taxi: conns[0].clone(),
        bus: conns[1].clone(),
        subway: conns[2].clone(),
        boat: conns[3].clone(),
    };
}

pub fn parse_file(filepath: &str) -> Vec<Station> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut stations = Vec::new();
    for line in reader.lines() {
        stations.push(parse_line(&line.unwrap()));
    }
    return stations;
}

pub fn create_adjacency_matrix(stations: Vec<Station>, mr_x: bool) -> Vec<Vec<u8>> {
    let mut matrix = vec![vec![0; stations.len() + 1]; stations.len() + 1];

    for (idx, station) in stations.iter().enumerate() {
        for taxi_st in &station.taxi {
            let taxi_idx = taxi_st.clone();
            matrix[idx][taxi_idx as usize] = 1;
        }
        for bus_st in &station.bus {
            let bus_idx = bus_st.clone();
            matrix[idx][bus_idx as usize] = 1;
        }
        for subway_st in &station.subway {
            let subway_idx = subway_st.clone();
            matrix[idx][subway_idx as usize] = 1;
        }
        if mr_x {
            for boat_st in &station.boat {
                let boat_idx = boat_st.clone();
                matrix[idx][boat_idx as usize] = 1;
            }
        }
    }

    return matrix;
}

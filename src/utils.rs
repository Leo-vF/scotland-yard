#[derive(Clone, Debug)]
pub enum Ticket {
    Taxi,
    Bus,
    Subway,
    Black,
}

#[derive(Debug, Clone)]
pub struct Station {
    pub num: u8,
    pub taxi: Vec<u8>,
    pub bus: Vec<u8>,
    pub subway: Vec<u8>,
    pub boat: Vec<u8>,
}

impl Station {
    pub fn find_possible_from(&self, stations: &Vec<Station>, tickets: Vec<Ticket>) -> Vec<u8> {
        let cur_ticket = match tickets.get(0) {
            Some(val) => val,
            None => return vec![],
        };
        if tickets.len() == 1 {
            return match cur_ticket {
                Ticket::Taxi => self.taxi.clone(),
                Ticket::Bus => self.bus.clone(),
                Ticket::Subway => self.subway.clone(),
                Ticket::Black => {
                    let mut taxi = self.taxi.clone();
                    taxi.extend(&self.bus);
                    taxi.extend(&self.subway);
                    taxi.extend(&self.boat);
                    taxi
                }
            };
        }
        let inner = |neighbors: &Vec<u8>| {
            neighbors
                .iter()
                .flat_map(|num| {
                    println!("{:?}", stations.get(*num as usize - 1).unwrap());
                    stations
                        .get(*num as usize - 1)
                        .unwrap()
                        .find_possible_from(&stations, tickets.clone().split_off(1))
                })
                .collect()
        };
        match cur_ticket {
            Ticket::Taxi => inner(&self.taxi),
            Ticket::Bus => inner(&self.bus),
            Ticket::Subway => inner(&self.subway),
            Ticket::Black => {
                let mut bytaxi = inner(&self.taxi);
                bytaxi.extend(inner(&self.bus));
                bytaxi.extend(inner(&self.subway));
                bytaxi.extend(inner(&self.boat));
                bytaxi
            }
        }
    }
}

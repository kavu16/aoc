use std::{collections::{HashMap, VecDeque}, ops::Not};

#[test]

fn test() {
    let example1 = String::from(
"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
    );

    let example2 = String::from(
"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
    );

    assert_eq!(solve1(&example1), 32_000_000);
    assert_eq!(solve1(&example2), 11_687_500);

    // assert_eq!(solve2(&example1), 0);
    // assert_eq!(solve2(&example2), 0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Strength {
    High,
    Low,
}
use Strength::*;

struct Pulse {
    strength: Strength,
    source: String,
    dest: String,
}

#[derive(Debug, Clone)]
enum Tower {
    Broadcaster {
        connections: Vec<String>,
    },
    FlipFlop {
        connections: Vec<String>,
        state: bool,
    },
    Conjunction {
        connections: Vec<String>,
        last: HashMap<String, Strength>,
    },
}
use Tower::*;

impl Tower {
    fn send_pulse(
        &mut self, 
        curr_strength: Strength, 
        curr_id: String, 
        prev_id: String, 
        pulse_queue: &mut VecDeque<Pulse>
    ) {
        match self {
            Broadcaster { connections } => {
                connections.iter().for_each(|d| {
                    let new_pulse = Pulse { 
                        strength: curr_strength, 
                        source: curr_id.to_string(), 
                        dest: d.to_string() 
                    };
                    pulse_queue.push_back(new_pulse);
                });
            }
            FlipFlop { connections, state } => {
                match curr_strength {
                    High => (),
                    Low => {
                        *state = state.not();
                        let mut new_strength = Low;
                        if *state { new_strength = High }
                        connections.iter().for_each(|d| {
                            let new_pulse = Pulse { 
                                strength: new_strength, 
                                source: curr_id.to_string(), 
                                dest: d.to_string() };
                            pulse_queue.push_back(new_pulse);
                        });
                    }
                }
            },
            Conjunction { connections, last } => {
                last.insert(prev_id.to_string(), curr_strength);
                let mut new_strength = High;
                if last.iter().all(|(_k, &s)| s == High ) { new_strength = Low; }
                connections.iter().for_each(|d| {
                    let new_pulse = Pulse { 
                        strength: new_strength, 
                        source: curr_id.to_string(), 
                        dest: d.to_string() };
                    pulse_queue.push_back(new_pulse);
                });
            }
        }
    }

    fn get_connections(&self) -> &Vec<String> {
        match self {
            Broadcaster { connections } => connections,
            FlipFlop { connections, state: _ } => connections,
            Conjunction { connections, last: _ } => connections,
        }
    }
}

struct Field {
    towers: HashMap<String, Tower>,
    pulse_queue: VecDeque<Pulse>,
    high_count: usize,
    low_count: usize,
    cycles: HashMap<String, usize>,
    presses: usize,
}

impl Field {
    fn press_button<const RX: bool>(&mut self) {
        self.presses += 1;
        // println!("Number of presses = {}", self.presses);
        let start_pulse = Pulse { strength: Low, source: String::from("button"), dest: String::from("broadcaster") };
        self.pulse_queue.push_front(start_pulse);
        while !self.pulse_queue.is_empty() {
            for _ in 0..self.pulse_queue.len() {
                let Pulse { 
                    strength: curr_strength, 
                    source: prev_id, 
                    dest: curr_id
                } = self.pulse_queue.pop_front().unwrap();
                // println!("Pulse strength: {}, pulse source: {prev_id}, pulse dest: {curr_id}", if curr_strength == High {"High"} else {"Low"});
                match curr_strength {
                    High => {
                        self.high_count += 1;
                        if RX && (
                            prev_id == String::from("fz") ||
                            prev_id == String::from("xf") ||
                            prev_id == String::from("mp") ||
                            prev_id == String::from("hn")
                        ) {
                            self.cycles.insert(prev_id.clone(), self.presses);
                            // println!("Cycles = {:?}", self.cycles);
                        }
                    }
                    Low => self.low_count += 1,
                }
                if let Some(curr_tower) = self.towers.get_mut(&curr_id) {
                    curr_tower.send_pulse(curr_strength, curr_id, prev_id, &mut self.pulse_queue);
                }
            }
        }
    }
}

pub fn solve1(data: &String) -> usize {
    let mut tower_field = Field { 
        towers: HashMap::<String, Tower>::new(),
        pulse_queue: VecDeque::<Pulse>::new(),
        high_count: 0,
        low_count: 0,
        cycles: HashMap::<String, usize>::from([
            (String::from("fz"), 0),
            (String::from("xf"), 0),
            (String::from("mp"), 0),
            (String::from("hn"), 0)
        ]),
        presses: 0,
    };
    
    for line in data.lines() {
        let (tower, connections) = line.split_once("->").unwrap();
        let tower = tower.trim();
        let connections = connections.trim().split(",").map(|c| c.trim().to_string()).collect::<Vec<String>>();

        if tower.contains("broadcaster") {
            tower_field.towers.insert(String::from("broadcaster"),
                                        Broadcaster { connections });
        } else if tower.contains("%") {
            tower_field.towers.insert(String::from(tower.trim_start_matches("%")),
                                        FlipFlop { connections, state: false });
        } else if tower.contains("&") {
            tower_field.towers.insert(String::from(tower.trim_start_matches("&")),
                                        Conjunction { connections, last: HashMap::<String, Strength>::new()});
        }

    }

    let mut prev_connections = HashMap::<String, Vec<(String, Strength)>>::new();
    for (id, ref tower) in &tower_field.towers {
        tower.get_connections().iter().for_each(|c_id| {
            if let Some(prev_ids) = prev_connections.get_mut(c_id) {
                prev_ids.push((id.to_string(), Low));
            } else {
                prev_connections.insert(c_id.to_string(), vec![(id.to_string(), Low)]);
            }
        })
    }

    for (id, prev_ids) in prev_connections {
        if let Some(tower) = tower_field.towers.get_mut(&id) {
            match tower {
                Conjunction { connections: _, last }=> {
                    last.extend(prev_ids);
                }
                _ => (),
            }
        }
    }

    for _ in 0..1000 {
        tower_field.press_button::<false>();
    }

    let res = tower_field.high_count * tower_field.low_count;
    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> usize {
    let mut tower_field = Field { 
        towers: HashMap::<String, Tower>::new(),
        pulse_queue: VecDeque::<Pulse>::new(),
        high_count: 0,
        low_count: 0,
        cycles: HashMap::<String, usize>::from([
            (String::from("fz"), 0),
            (String::from("xf"), 0),
            (String::from("mp"), 0),
            (String::from("hn"), 0),
        ]),
        presses: 0,
    };
    
    for line in data.lines() {
        let (tower, connections) = line.split_once("->").unwrap();
        let tower = tower.trim();
        let connections = connections.trim().split(",").map(|c| c.trim().to_string()).collect::<Vec<String>>();

        if tower.contains("broadcaster") {
            tower_field.towers.insert(String::from("broadcaster"),
                                        Broadcaster { connections });
        } else if tower.contains("%") {
            tower_field.towers.insert(String::from(tower.trim_start_matches("%")),
                                        FlipFlop { connections, state: false });
        } else if tower.contains("&") {
            tower_field.towers.insert(String::from(tower.trim_start_matches("&")),
                                        Conjunction { connections, last: HashMap::<String, Strength>::new()});
        }

    }

    let mut prev_connections = HashMap::<String, Vec<(String, Strength)>>::new();
    for (id, ref tower) in &tower_field.towers {
        tower.get_connections().iter().for_each(|c_id| {
            prev_connections.entry(c_id.clone())
                .and_modify(|prev_ids| prev_ids.push((id.to_string(), Low)))
                .or_insert(vec![(id.to_string(), Low)]);
        })
    }

    for (id, prev_ids) in prev_connections {
        if let Some(tower) = tower_field.towers.get_mut(&id) {
            match tower {
                Conjunction { connections: _, last } => {
                    last.extend(prev_ids);
                }
                _ => (),
            }
        }
    }

    loop {
        tower_field.press_button::<true>();
        if tower_field.cycles.values().all(|&p| p > 0) {
            break;
        }
    }

    let res = tower_field.cycles.values().product();
    println!("Part 2 = {}", res);
    res
}

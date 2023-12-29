use std::collections::{HashMap, VecDeque};

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

#[derive(Clone, Copy, PartialEq, Eq)]
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

enum TowerType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}
use TowerType::*;

struct Tower {
    connections: Vec<String>,
    ttype: TowerType,
    state: bool,
    last: HashMap<String, Strength>,
}

impl Tower {
    fn send_pulse(&mut self, 
                    curr_strength: Strength, 
                    curr_id: String, 
                    prev_id: String, 
                    pulse_queue: &mut VecDeque<Pulse>) {
        match self.ttype {
            Broadcaster => {
                self.connections.iter().for_each(|d| {
                    let new_pulse = Pulse { strength: curr_strength, 
                                                   source: curr_id.to_string(), 
                                                   dest: d.to_string() };
                    pulse_queue.push_back(new_pulse);
                });
            }
            FlipFlop => {
                match curr_strength {
                    High => (),
                    Low => {
                        self.state = !self.state;
                        let mut new_strength = Low;
                        if self.state { new_strength = High }
                        self.connections.iter().for_each(|d| {
                            let new_pulse = Pulse { strength: new_strength, 
                                                           source: curr_id.to_string(), 
                                                           dest: d.to_string() };
                            pulse_queue.push_back(new_pulse);
                        });
                    }
                }
            },
            Conjunction => {
                self.last.insert(prev_id.to_string(), curr_strength);
                let mut new_strength = High;
                if self.last.iter().all(|(_k, &s)| s == High ) { new_strength = Low; }
                self.connections.iter().for_each(|d| {
                    let new_pulse = Pulse { strength: new_strength, 
                                                source: curr_id.to_string(), 
                                                dest: d.to_string() };
                    pulse_queue.push_back(new_pulse);
                });
            }
        }
    }
}

struct Field {
    towers: HashMap<String, Tower>,
    pulse_queue: VecDeque<Pulse>,
    high_count: usize,
    low_count: usize,
}

impl Field {
    fn press_button<const RX: bool>(&mut self) -> bool {
        let start_pulse = Pulse { strength: Low, source: String::from("button"), dest: String::from("broadcaster") };
        self.pulse_queue.push_front(start_pulse);
        while !self.pulse_queue.is_empty() {
            for _ in 0..self.pulse_queue.len() {
                let Pulse { strength: curr_strength, 
                            source: prev_id, 
                            dest: curr_id } = self.pulse_queue.pop_front().unwrap();
                // println!("Pulse strength: {}, pulse source: {prev_id}, pulse dest: {curr_id}", if curr_strength == High {"High"} else {"Low"});
                match curr_strength {
                    High => self.high_count += 1,
                    Low => {
                        self.low_count += 1;
                        if RX && curr_id == String::from("rx") { return true; }
                    }
                }
                if let Some(curr_tower) = self.towers.get_mut(&curr_id) {
                    curr_tower.send_pulse(curr_strength, curr_id, prev_id, &mut self.pulse_queue);
                } else {
                    continue;
                }
            }
        }
        false
    }
}

pub fn solve1(data: &String) -> usize {
    let mut tower_field = Field { towers: HashMap::<String, Tower>::new(),
                                     pulse_queue: VecDeque::<Pulse>::new(),
                                     high_count: 0,
                                     low_count: 0 };
    
    for line in data.lines() {
        let (tower, connections) = line.split_once("->").unwrap();
        let tower = tower.trim();
        let connections = connections.trim().split(",").map(|c| c.trim().to_string()).collect::<Vec<String>>();

        if tower.contains("broadcaster") {
            tower_field.towers.insert(String::from("broadcaster"),
                                        Tower { connections, 
                                                   ttype: Broadcaster, 
                                                   state: true, 
                                                   last: HashMap::<String, Strength>::new() });
        } else if tower.contains("%") {
            tower_field.towers.insert(String::from(tower.trim_start_matches("%")),
                                        Tower { connections, 
                                                   ttype: FlipFlop, 
                                                   state: false, 
                                                   last: HashMap::<String, Strength>::new() });
        } else if tower.contains("&") {
            tower_field.towers.insert(String::from(tower.trim_start_matches("&")),
                                        Tower { connections, 
                                                   ttype: Conjunction, 
                                                   state: true, 
                                                   last:  HashMap::<String, Strength>::new()});
        }

    }

    let mut prev_connections = HashMap::<String, Vec<(String, Strength)>>::new();
    for (id, tower) in &tower_field.towers {
        tower.connections.iter().for_each(|c_id| {
            if let Some(prev_ids) = prev_connections.get_mut(c_id) {
                prev_ids.push((id.to_string(), Low));
            } else {
                prev_connections.insert(c_id.to_string(), vec![(id.to_string(), Low)]);
            }
        })
    }

    for (id, prev_ids) in prev_connections {
        if let Some(tower) = tower_field.towers.get_mut(&id) {
            match tower.ttype {
                Conjunction => {
                    tower.last.extend(prev_ids);
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
    let mut tower_field = Field { towers: HashMap::<String, Tower>::new(),
                                     pulse_queue: VecDeque::<Pulse>::new(),
                                     high_count: 0,
                                     low_count: 0 };
    
    for line in data.lines() {
        let (tower, connections) = line.split_once("->").unwrap();
        let tower = tower.trim();
        let connections = connections.trim().split(",").map(|c| c.trim().to_string()).collect::<Vec<String>>();

        if tower.contains("broadcaster") {
            tower_field.towers.insert(String::from("broadcaster"),
                                        Tower { connections, 
                                                   ttype: Broadcaster, 
                                                   state: true, 
                                                   last: HashMap::<String, Strength>::new() });
        } else if tower.contains("%") {
            tower_field.towers.insert(String::from(tower.trim_start_matches("%")),
                                        Tower { connections, 
                                                   ttype: FlipFlop, 
                                                   state: false, 
                                                   last: HashMap::<String, Strength>::new() });
        } else if tower.contains("&") {
            tower_field.towers.insert(String::from(tower.trim_start_matches("&")),
                                        Tower { connections, 
                                                   ttype: Conjunction, 
                                                   state: true, 
                                                   last:  HashMap::<String, Strength>::new()});
        }

    }

    let mut prev_connections = HashMap::<String, Vec<(String, Strength)>>::new();
    for (id, tower) in &tower_field.towers {
        tower.connections.iter().for_each(|c_id| {
            if let Some(prev_ids) = prev_connections.get_mut(c_id) {
                prev_ids.push((id.to_string(), Low));
            } else {
                prev_connections.insert(c_id.to_string(), vec![(id.to_string(), Low)]);
            }
        })
    }

    for (id, prev_ids) in prev_connections {
        if let Some(tower) = tower_field.towers.get_mut(&id) {
            match tower.ttype {
                Conjunction => {
                    tower.last.extend(prev_ids);
                }
                _ => (),
            }
        }
    }

    let mut res = 0;
    // for i in 1.. {
    //     if i % 10000 == 0 { println!("{i}"); }
    //     if tower_field.press_button::<true>() {
    //         res = i;
    //     }
    // }

    println!("Part 1 = {}", res);
    res
}

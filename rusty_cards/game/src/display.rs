use crate::card::Minion;

// constants
pub const DECK_DISPLAY_WIDTH: usize = 15;
pub const CARD_DISPLAY_WIDTH: usize = 15;

// Single display
pub fn display_string_center(string_size: usize, s: &str) -> String {
    let free_space = string_size - s.len();
    let before = free_space / 2;
    let after = (free_space + 1) / 2;

    let mut display = String::new();
    for _ in 0..before {
        display += " ";
    }
    display += s;
    for _ in 0..after {
        display += " ";
    }
    display
}

pub fn display_string_left(string_size: usize, s: &str) -> String {
    let free_space = string_size - s.len();
    let mut display: String = s.to_string();
    for _ in 0..free_space {
        display += " ";
    }
    display
}

pub fn display_string_right(string_size: usize, s: &str) -> String {
    let free_space = string_size - s.len();
    let mut display = String::new();
    for _ in 0..free_space {
        display += " ";
    }
    display += s;
    display
}

pub fn display_bound_center(string_size: usize, s: &str) -> String {
    let mut display = String::new();
    display += "|";
    display += &display_string_center(string_size, s);
    display += "|";

    display
}

pub fn display_bound_left(string_size: usize, s: &str) -> String {
    let mut display = String::new();
    display += "|";
    display += &display_string_left(string_size, s);
    display += "|";

    display
}

pub fn display_bound_right(string_size: usize, s: &str) -> String {
    let mut display = String::new();
    display += "|";
    display += &display_string_right(string_size, s);
    display += "|";

    display
}

pub fn display_edge(edge_size: usize) -> String {
    let mut display = String::new();
    display += "+";
    for _ in 0..edge_size {
        display += "-";
    }
    display += "+";

    display
}

pub fn display_edge_num(edge_size: usize, num: usize) -> String {
    let mut display = String::new();
    display += "+";

    let free_space = edge_size - num.to_string().len();
    let before = free_space / 2;
    let after = (free_space + 1) / 2;

    for _ in 0..before {
        display += "-";
    }

    display += &num.to_string();

    for _ in 0..after {
        display += "-";
    }
    display += "+";

    display
}

// Display for multiple objects

pub fn display_from_vec_center(part_size: usize, strings: Vec<String>) -> String {
    let mut display = String::new();
    for s in strings {
        display += &display_bound_center(part_size, &s);
    }
    display
}

pub fn display_from_vec_left(part_size: usize, strings: Vec<String>) -> String {
    let mut display = String::new();
    for s in strings {
        display += &display_bound_left(part_size, &s);
    }
    display
}

pub fn display_from_vec_right(part_size: usize, strings: Vec<String>) -> String {
    let mut display = String::new();
    for s in strings {
        display += &display_bound_right(part_size, &s);
    }
    display
}

pub fn display_edges(edge_size: usize, num_edges: usize) -> String {
    let mut display = String::new();
    for _ in 0..num_edges {
        display += &display_edge(edge_size);
    }
    display
}

pub fn display_edges_num(edge_size: usize, num_edges: usize) -> String {
    let mut display = String::new();
    for i in 1..=num_edges {
        display += &display_edge_num(edge_size, i);
    }
    display
}

// Minion display utils
fn get_mana(minions: &Vec<Option<&Minion>>) -> (Vec<String>, Vec<String>) {
    let mut upper = Vec::new();
    let mut lower = Vec::new();
    for minion in minions {
        match minion {
            Some(m) => {
                upper.push("Mana:".to_string());
                lower.push(m.get_mana().to_string());
            }
            None => {
                upper.push("".to_string());
                lower.push("".to_string());
            }
        };
    }
    (upper, lower)
}

fn get_names(minions: &Vec<Option<&Minion>>) -> (Vec<String>, Vec<String>) {
    let mut upper = Vec::new();
    let mut lower = Vec::new();
    for minion in minions {
        match minion {
            Some(m) => {
                upper.push("Name:".to_string());
                lower.push(m.get_name().to_string());
            }
            None => {
                upper.push("".to_string());
                lower.push("".to_string());
            }
        };
    }
    (upper, lower)
}

fn get_attacks(minions: &Vec<Option<&Minion>>) -> (Vec<String>, Vec<String>) {
    let mut upper = Vec::new();
    let mut lower = Vec::new();
    for minion in minions {
        match minion {
            Some(m) => {
                upper.push("Attack:".to_string());
                lower.push(m.get_attack().to_string());
            }
            None => {
                upper.push("".to_string());
                lower.push("".to_string());
            }
        };
    }
    (upper, lower)
}

fn get_healths(minions: &Vec<Option<&Minion>>) -> (Vec<String>, Vec<String>) {
    let mut upper = Vec::new();
    let mut lower = Vec::new();
    for minion in minions {
        match minion {
            Some(m) => {
                upper.push("Health:".to_string());
                lower.push(m.get_health().to_string());
            }
            None => {
                upper.push("".to_string());
                lower.push("".to_string());
            }
        };
    }
    (upper, lower)
}

pub enum EdgeNum {
    Low,
    Up,
    None,
}

pub fn display_card_row(minions: &Vec<Option<&Minion>>, display_mana: bool, edge_num: EdgeNum) {
    // DISPLAYING UPPER EDGE
    let edge = match edge_num {
        EdgeNum::Up => display_edges_num(CARD_DISPLAY_WIDTH, minions.len()),
        EdgeNum::Low => display_edges(CARD_DISPLAY_WIDTH, minions.len()),
        EdgeNum::None => display_edges(CARD_DISPLAY_WIDTH, minions.len()),
    };
    println!("{}", edge);

    // DISPLAYING MANA
    if display_mana {
        let (vec_up, vec_low) = get_mana(minions);
        println!("{}", display_from_vec_left(CARD_DISPLAY_WIDTH, vec_up));
        println!("{}", display_from_vec_center(CARD_DISPLAY_WIDTH, vec_low));
    }

    // DISPLAYING NAMES
    let (vec_up, vec_low) = get_names(minions);
    println!("{}", display_from_vec_left(CARD_DISPLAY_WIDTH, vec_up));
    println!("{}", display_from_vec_center(CARD_DISPLAY_WIDTH, vec_low));

    // DISPLAYING ATTACKS
    let (vec_up, vec_low) = get_attacks(minions);
    println!("{}", display_from_vec_left(CARD_DISPLAY_WIDTH, vec_up));
    println!("{}", display_from_vec_center(CARD_DISPLAY_WIDTH, vec_low));

    // DISPLAYING HEALTHS
    let (vec_up, vec_low) = get_healths(minions);
    println!("{}", display_from_vec_left(CARD_DISPLAY_WIDTH, vec_up));
    println!("{}", display_from_vec_center(CARD_DISPLAY_WIDTH, vec_low));

    // DISPLAYING LOWER EDGE
    let edge = match edge_num {
        EdgeNum::Up => display_edges(CARD_DISPLAY_WIDTH, minions.len()),
        EdgeNum::Low => display_edges_num(CARD_DISPLAY_WIDTH, minions.len()),
        EdgeNum::None => display_edges(CARD_DISPLAY_WIDTH, minions.len()),
    };
    println!("{}", edge);
}

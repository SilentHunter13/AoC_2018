use core::cmp::min;
use regex::Regex;
use std::cmp::Reverse;
use std::fs;

#[derive(Debug)]
struct Group {
    index: usize, //nur zur Unterscheidung
    units: u32,
    hitpoints: u32,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
    damage: u32,
    damage_type: String,
    initiative: u32,
    infection: bool,
}

pub fn star_1() -> u32 {
    let contents =
        fs::read_to_string("./input/day_24.txt").expect("Something went wrong reading the file");

    let mut groups = read_input(contents);

    loop {
        //absteigend Sortieren nach Reihenfolge der Target Wahl
        groups.sort_by_key(|x| Reverse(x.initiative));
        groups.sort_by_key(|x| Reverse(get_effective_power(&x)));

        let fights = select_target(&groups);

        if fights.is_empty() {
            break;
        }

        perform_battle(&fights, &mut groups);
    }
    println!("{:?}", groups);
    groups.iter().fold(0, |sum, x| sum + x.units)
}

fn perform_battle(fights: &[(usize, usize, u32)], groups: &mut Vec<Group>) {
    for fight in fights {
        let damage = get_damage(&groups[fight.0], &groups[fight.1]);

        let lost_units = min(damage / groups[fight.1].hitpoints, groups[fight.1].units);

        // println!(
        //     "{} v {} killing {}",
        //     groups[fight.0].index, groups[fight.1].index, lost_units
        // );

        groups[fight.1].units -= lost_units;
    }
    groups.retain(|x| x.units > 0);
}

//Erwartet, dass die Gruppen nach Wahlreihenfolge sortiert sind
fn select_target(groups: &[Group]) -> Vec<(usize, usize, u32)> {
    let mut fights: Vec<(usize, usize, u32)> = Vec::new();

    for (a_index, attacker) in groups.iter().enumerate() {
        let mut best_opponent_index: Option<usize> = None;
        for (d_index, defender) in groups.iter().enumerate() {
            //nicht gegen sichselbst und nur gegen Gegner kämpfen
            if attacker.index != defender.index && attacker.infection != defender.infection {
                let this_damage = get_damage(attacker, defender);
                if let Some(best_opponent_index_) = best_opponent_index {
                    let best_damage = get_damage(attacker, &groups[best_opponent_index_]);
                    if this_damage > best_damage
                        || (this_damage == best_damage
                            && get_effective_power(defender)
                                > get_effective_power(&groups[best_opponent_index_]))
                        || (get_effective_power(defender)
                            == get_effective_power(&groups[best_opponent_index_])
                            && defender.initiative > groups[best_opponent_index_].initiative)
                    {
                        best_opponent_index = Some(d_index);
                    }
                } else if this_damage > 0 {
                    best_opponent_index = Some(d_index);
                }
            }
        }

        if let Some(best_opponent_index) = best_opponent_index {
            //Jedes Ziel darf nur einmal gewählt werden
            let already_selected = fights.iter().any(|x| (x.1 == best_opponent_index));
            if !already_selected {
                let fight = (a_index, best_opponent_index, attacker.initiative);
                fights.push(fight);
            }
        }
    }
    fights.sort_by_key(|x| Reverse(x.2));
    fights
}

fn get_effective_power(group: &Group) -> u32 {
    group.units * group.damage
}

fn get_damage(attacker: &Group, defender: &Group) -> u32 {
    if defender.weaknesses.contains(&attacker.damage_type) {
        2 * get_effective_power(attacker)
    } else if defender.immunities.contains(&attacker.damage_type) {
        0
    } else {
        get_effective_power(attacker)
    }
}

fn read_input(input: String) -> Vec<Group> {
    //1 units; 2 hitpoints; 3 weaknesses, immunities; 4 damage; 5 damage type; 6 initiative
    let group_re = Regex::new(
        "([0-9]+)[^0-9]+([0-9]+)[^0-9(]+(?:\\((.+)\\))?[^0-9]+([0-9]+) ([a-z]+) [^0-9]+([0-9]+)",
    )
    .unwrap();

    //1 immunities
    let immunities_re = Regex::new("immune to ([^;\n]+)").unwrap();

    //1 weaknesses
    let weaknesses_re = Regex::new("weak to ([^;\n]+)").unwrap();

    let mut infection = false;
    let mut groups: Vec<Group> = Vec::new();

    for line in input.lines() {
        if let Some(group_match) = group_re.captures(line) {
            let mut new_group = Group {
                index: groups.len(),
                units: group_match[1].parse::<u32>().unwrap(),
                hitpoints: group_match[2].parse::<u32>().unwrap(),
                weaknesses: Vec::new(),
                immunities: Vec::new(),
                damage: group_match[4].parse::<u32>().unwrap(),
                damage_type: group_match[5].to_string(),
                initiative: group_match[6].parse::<u32>().unwrap(),
                infection,
            };

            if group_match.get(3).is_some() {
                if let Some(immunities_match) = immunities_re.captures(&group_match[3]) {
                    new_group.immunities = immunities_match[1]
                        .to_string()
                        .split(',')
                        .map(|x| x.trim().to_string())
                        .collect();
                }

                if let Some(weaknesses_match) = weaknesses_re.captures(&group_match[3]) {
                    new_group.weaknesses = weaknesses_match[1]
                        .to_string()
                        .split(',')
                        .map(|x| x.trim().to_string())
                        .collect();
                }
            }
            groups.push(new_group);
        } else {
            infection = line.contains("Infection");
        }
    }
    groups
}

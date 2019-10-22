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

    let groups = perform_battle(&contents, 0);
    groups.iter().fold(0, |sum, x| sum + x.units)
}

pub fn star_2() -> u32 {
    let contents =
        fs::read_to_string("./input/day_24.txt").expect("Something went wrong reading the file");

    let mut boost_value = 0;

    loop {
        let groups = perform_battle(&contents, boost_value);
        if groups.iter().all(|x| !x.infection) {
            break groups.iter().fold(0, |sum, x| sum + x.units);
        }
        boost_value += 1;
    }
}

fn perform_battle(contents: &str, boost_value: u32) -> Vec<Group> {
    let mut groups = read_input(&contents, boost_value);

    loop {
        //absteigend Sortieren nach Reihenfolge der Target Wahl
        groups.sort_by_key(|x| Reverse(x.initiative));
        groups.sort_by_key(|x| Reverse(get_effective_power(&x)));
        let fights = select_target(&groups);

        let units_lost = perform_figts(&fights, &mut groups);

        if fights.is_empty() || !units_lost {
            break groups;
        }
    }
}

fn perform_figts(fights: &[(usize, usize, u32)], groups: &mut Vec<Group>) -> bool {
    let mut units_lost = false;
    for fight in fights {
        let damage = get_damage(&groups[fight.0], &groups[fight.1]);

        let lost_units = min(damage / groups[fight.1].hitpoints, groups[fight.1].units);

        units_lost = units_lost || lost_units > 0;

        groups[fight.1].units -= lost_units;
    }
    groups.retain(|x| x.units > 0);
    units_lost
}

//Erwartet, dass die Gruppen nach Wahlreihenfolge sortiert sind
fn select_target(groups: &[Group]) -> Vec<(usize, usize, u32)> {
    let mut fights: Vec<(usize, usize, u32)> = Vec::new();

    for (a_index, attacker) in groups.iter().enumerate() {
        let mut best_opponent: Option<usize> = None;
        for (d_index, defender) in groups
            .iter()
            .enumerate()
            //nur gegen Gegner kämpfen
            .filter(|(_, x)| x.infection != attacker.infection)
        {
            let this_damage = get_damage(attacker, defender);
            if let Some(best_opponent_index) = best_opponent {
                let best_damage = get_damage(attacker, &groups[best_opponent_index]);

                if this_damage > best_damage
                    || (this_damage == best_damage
                        && (get_effective_power(defender)
                            > get_effective_power(&groups[best_opponent_index])
                            || (get_effective_power(defender)
                                == get_effective_power(&groups[best_opponent_index])
                                && defender.initiative > groups[best_opponent_index].initiative)))
                {
                    //Jedes Ziel darf nur einmal gewählt werden
                    let already_selected = fights.iter().any(|x| (x.1 == d_index));
                    if !already_selected {
                        best_opponent = Some(d_index);
                    }
                }
            } else if this_damage > 0 {
                //Jedes Ziel darf nur einmal gewählt werden
                let already_selected = fights.iter().any(|x| (x.1 == d_index));
                if !already_selected {
                    best_opponent = Some(d_index);
                }
            }
        }

        if let Some(best_opponent_index) = best_opponent {
            let fight = (a_index, best_opponent_index, attacker.initiative);
            fights.push(fight);
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

fn read_input(input: &str, boost_value: u32) -> Vec<Group> {
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
    let mut boost = boost_value;
    let mut groups: Vec<Group> = Vec::new();

    for line in input.lines() {
        if let Some(group_match) = group_re.captures(line) {
            let mut new_group = Group {
                index: groups.len(),
                units: group_match[1].parse::<u32>().unwrap(),
                hitpoints: group_match[2].parse::<u32>().unwrap(),
                weaknesses: Vec::new(),
                immunities: Vec::new(),
                damage: group_match[4].parse::<u32>().unwrap() + boost,
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
            if infection {
                boost = 0;
            }
        }
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_damage() {
        let attacker = Group {
            index: 0,
            units: 1,
            hitpoints: 5,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 1,
            infection: false,
        };
        let defender = Group {
            index: 1,
            units: 1,
            hitpoints: 10,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 5,
            damage_type: String::from("cold"),
            initiative: 2,
            infection: true,
        };

        let result = get_damage(&attacker, &defender);

        assert_eq!(result, 10);
    }

    #[test]
    fn test_get_damage_weakness() {
        let attacker = Group {
            index: 0,
            units: 1,
            hitpoints: 5,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 1,
            infection: false,
        };
        let defender = Group {
            index: 1,
            units: 1,
            hitpoints: 10,
            weaknesses: vec![String::from("cold")],
            immunities: Vec::new(),
            damage: 5,
            damage_type: String::from("cold"),
            initiative: 2,
            infection: true,
        };

        let result = get_damage(&attacker, &defender);

        assert_eq!(result, 20);
    }

    #[test]
    fn test_get_damage_immunity() {
        let attacker = Group {
            index: 0,
            units: 1,
            hitpoints: 5,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 1,
            infection: false,
        };
        let defender = Group {
            index: 1,
            units: 1,
            hitpoints: 10,
            weaknesses: Vec::new(),
            immunities: vec![String::from("cold")],
            damage: 5,
            damage_type: String::from("cold"),
            initiative: 2,
            infection: true,
        };

        let result = get_damage(&attacker, &defender);

        assert_eq!(result, 0);
    }

    #[test]
    fn test_select_target_same_damage() {
        let group1 = Group {
            index: 0,
            units: 3,
            hitpoints: 5,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 1,
            infection: false,
        };
        let group2 = Group {
            index: 1,
            units: 2,
            hitpoints: 10,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 2,
            infection: true,
        };
        let group3 = Group {
            index: 1,
            units: 1,
            hitpoints: 10,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 3,
            infection: true,
        };

        let groups = vec![group1, group2, group3];

        let result = select_target(&groups);

        let expected = vec![(1, 0, 2), (0, 1, 1)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_select_target_same_effective_power() {
        let group1 = Group {
            index: 0,
            units: 3,
            hitpoints: 5,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 1,
            infection: false,
        };
        let group2 = Group {
            index: 1,
            units: 2,
            hitpoints: 10,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 2,
            infection: true,
        };
        let group3 = Group {
            index: 1,
            units: 2,
            hitpoints: 10,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 3,
            infection: true,
        };

        let groups = vec![group1, group2, group3];

        let result = select_target(&groups);

        let expected = vec![(1, 0, 2), (0, 2, 1)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_select_target() {
        let group1 = Group {
            index: 0,
            units: 3,
            hitpoints: 5,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 1,
            infection: false,
        };
        let group2 = Group {
            index: 1,
            units: 2,
            hitpoints: 10,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 2,
            infection: true,
        };
        let group3 = Group {
            index: 1,
            units: 2,
            hitpoints: 10,
            weaknesses: vec![String::from("cold")],
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 2,
            infection: true,
        };

        let groups = vec![group1, group2, group3];

        let result = select_target(&groups);

        let expected = vec![(1, 0, 2), (0, 2, 1)];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_select_target_already_selectec() {
        let group1 = Group {
            index: 0,
            units: 3,
            hitpoints: 5,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 4,
            infection: false,
        };
        let group4 = Group {
            index: 3,
            units: 3,
            hitpoints: 5,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 1,
            infection: false,
        };
        let group2 = Group {
            index: 1,
            units: 2,
            hitpoints: 10,
            weaknesses: Vec::new(),
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 3,
            infection: true,
        };
        let group3 = Group {
            index: 2,
            units: 2,
            hitpoints: 10,
            weaknesses: vec![String::from("cold")],
            immunities: Vec::new(),
            damage: 10,
            damage_type: String::from("cold"),
            initiative: 2,
            infection: true,
        };

        let groups = vec![group1, group4, group2, group3];

        let result = select_target(&groups);

        let expected = vec![(0, 3, 4), (2, 0, 3), (3, 1, 2), (1, 2, 1)];

        assert_eq!(result, expected);
    }
}

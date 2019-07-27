use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Event {
    Sleep,
    Awake,
    Shift(u16),
}

#[derive(Debug)]
struct Record {
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    event: Event,
}

pub fn star_1() -> u32 {
    //1 month; 2 day; 3 hour; 4 minute; 5 event
    let record_re = Regex::new(".*1518-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)] (.+)").unwrap();
    let shift_re = Regex::new("Guard #([0-9]+) begins shift").unwrap();

    let contents =
        fs::read_to_string("./input/day_4.txt").expect("Something went wrong reading the file");

    let mut records: Vec<Record> = Vec::new();

    for line in contents.lines() {
        let record_match = record_re.captures(line).unwrap();

        let record = Record {
            month: record_match[1].parse::<u8>().unwrap(),
            day: record_match[2].parse::<u8>().unwrap(),
            hour: record_match[3].parse::<u8>().unwrap(),
            minute: record_match[4].parse::<u8>().unwrap(),
            event: match &record_match[5] {
                "wakes up" => Event::Awake,
                "falls asleep" => Event::Sleep,
                _ => {
                    let shift_match = shift_re.captures(line).unwrap();
                    Event::Shift(shift_match[1].parse::<u16>().unwrap())
                }
            },
        };
        records.push(record);
    }

    records.sort_by_key(|t| t.minute);
    records.sort_by_key(|t| t.hour);
    records.sort_by_key(|t| t.day);
    records.sort_by_key(|t| t.month);

    let mut guard_on_duty = 0;
    let mut sleep_start = 0;
    type MinuteMap = HashMap<u8, u32>;
    let mut guards: HashMap<u16, MinuteMap> = HashMap::new();
    for record in records.iter() {
        match record.event {
            Event::Shift(guard) => guard_on_duty = guard,
            Event::Sleep => {
                if record.hour == 0 {
                    sleep_start = record.minute;
                }
            }
            Event::Awake => {
                if record.hour == 0 {
                    for minute in sleep_start..record.minute {
                        *guards
                            .entry(guard_on_duty)
                            .or_insert_with(HashMap::new)
                            .entry(minute)
                            .or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let mut max_guard: u16 = 0;
    let mut max_minutes = 0;
    let mut max_minute: u8 = 0;
    for guard in guards.iter() {
        let minutes_sum: u32 = guard.1.values().sum();

        if minutes_sum > max_minutes {
            max_guard = *guard.0;
            max_minutes = minutes_sum;

            let mut max_value: u32 = 0;
            for value in guard.1.iter() {
                if *value.1 > max_value {
                    max_value = *value.1;
                    max_minute = *value.0;
                }
            }
        }
    }
    u32::from(max_guard) * u32::from(max_minute)
}

pub fn star_2() -> u32 {
    //1 month; 2 day; 3 hour; 4 minute; 5 event
    let record_re = Regex::new(".*1518-([0-9]+)-([0-9]+) ([0-9]+):([0-9]+)] (.+)").unwrap();
    let shift_re = Regex::new("Guard #([0-9]+) begins shift").unwrap();

    let contents =
        fs::read_to_string("./input/day_4.txt").expect("Something went wrong reading the file");

    let mut records: Vec<Record> = Vec::new();

    for line in contents.lines() {
        let record_match = record_re.captures(line).unwrap();

        let record = Record {
            month: record_match[1].parse::<u8>().unwrap(),
            day: record_match[2].parse::<u8>().unwrap(),
            hour: record_match[3].parse::<u8>().unwrap(),
            minute: record_match[4].parse::<u8>().unwrap(),
            event: match &record_match[5] {
                "wakes up" => Event::Awake,
                "falls asleep" => Event::Sleep,
                _ => {
                    let shift_match = shift_re.captures(line).unwrap();
                    Event::Shift(shift_match[1].parse::<u16>().unwrap())
                }
            },
        };
        records.push(record);
    }

    records.sort_by_key(|t| t.minute);
    records.sort_by_key(|t| t.hour);
    records.sort_by_key(|t| t.day);
    records.sort_by_key(|t| t.month);

    let mut guard_on_duty = 0;
    let mut sleep_start = 0;
    type MinuteMap = HashMap<u8, u32>;
    let mut guards: HashMap<u16, MinuteMap> = HashMap::new();
    for record in records.iter() {
        match record.event {
            Event::Shift(guard) => guard_on_duty = guard,
            Event::Sleep => {
                if record.hour == 0 {
                    sleep_start = record.minute;
                }
            }
            Event::Awake => {
                if record.hour == 0 {
                    for minute in sleep_start..record.minute {
                        *guards
                            .entry(guard_on_duty)
                            .or_insert_with(HashMap::new)
                            .entry(minute)
                            .or_insert(0) += 1;
                    }
                }
            }
        }
    }

    let mut max_guard: u16 = 0;
    let mut max_minute: u8 = 0;
    let mut max_sleeps: u32 = 0;
    for guard in guards.iter() {
        for value in guard.1.iter() {
            if *value.1 > max_sleeps {
                max_sleeps = *value.1;
                max_minute = *value.0;
                max_guard = *guard.0;
            }
        }
    }
    u32::from(max_guard) * u32::from(max_minute)
}

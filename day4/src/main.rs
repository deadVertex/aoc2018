use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[macro_use]
extern crate scan_fmt;

extern crate chrono;
use chrono::prelude::*;

#[derive(Copy, Clone)]
enum EventType {
    BeginsShift,
    FallsAsleep,
    WakesUp,
}

#[derive(Copy, Clone)]
struct Event {
    event_type: EventType,
    timestamp: DateTime<Utc>,
    id: i32,
}

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    // Parse each line of input
    let mut events: Vec<Event> = s
        .lines()
        .map(|line| {
            let (year, month, day, hours, minutes, description) = scan_fmt!(
                line,
                "[{d}-{d}-{d} {d}:{d}] {[^\n]}",
                u32,
                u32,
                u32,
                u32,
                u32,
                String
            );
            let date = Utc
                .ymd(year.unwrap() as i32, month.unwrap(), day.unwrap())
                .and_hms(hours.unwrap(), minutes.unwrap(), 0);

            let text = description.unwrap();
            let first_char = text.chars().nth(0);

            // Convert string events to enum
            let event: Option<EventType> = match first_char {
                Some('G') => Some(EventType::BeginsShift),
                Some('f') => Some(EventType::FallsAsleep),
                Some('w') => Some(EventType::WakesUp),
                Some(_) => None,
                None => None,
            };

            // Parse guard id
            let guard_id: i32 = match event {
                Some(EventType::BeginsShift) => {
                    scan_fmt!(&text, "Guard #{d} begins shift", i32).unwrap()
                }
                Some(_) => -1,
                None => -1,
            };

            Event {
                event_type: event.unwrap(),
                timestamp: date,
                id: guard_id,
            }
        }).collect();

    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    // Populate id field for all events
    let mut guard_on_duty: Option<i32> = None;
    for event in &mut events {
        match event.event_type {
            EventType::BeginsShift => {
                guard_on_duty = Some(event.id);
            }
            EventType::FallsAsleep => {
                event.id = guard_on_duty.unwrap();
            }
            EventType::WakesUp => {
                event.id = guard_on_duty.unwrap();
            }
        }
    }

    // Collect all guard ids
    let mut unique_ids: Vec<i32> = events.iter().map(|event| event.id).collect();

    // Need to sort all elements to remove all duplicates
    unique_ids.sort();
    unique_ids.dedup();

    let mut guard_sleep_times: Vec<(i32, i64)> = Vec::new();

    // Calculate minutes asleep for each guard
    for id in &unique_ids {
        let events_for_guard = events.iter().filter(|event| event.id == *id);

        let mut total_minutes_asleep: i64 = 0;
        let mut sleep_begin: Option<DateTime<Utc>> = None;
        for event in events_for_guard {
            match event.event_type {
                EventType::FallsAsleep => {
                    sleep_begin = Some(event.timestamp);
                }
                EventType::WakesUp => {
                    let duration = event.timestamp - sleep_begin.unwrap();
                    total_minutes_asleep += duration.num_minutes();
                    sleep_begin = None;
                }
                EventType::BeginsShift => {}
            }
        }

        guard_sleep_times.push((*id, total_minutes_asleep));
    }

    // Find guard who spent most time sleeping
    guard_sleep_times.sort_by_key(|pair| pair.1);

    let sleepiest_guard = guard_sleep_times.last().unwrap().0;
    let events_for_guard: Vec<Event> = events
        .iter()
        .filter(|event| event.id == sleepiest_guard)
        .map(|event| *event)
        .collect();

    // Collect all of the minutes the the sleepiest guard spends asleep
    let mut minutes: Vec<u32> = Vec::new();
    let mut sleep_begin: Option<DateTime<Utc>> = None;
    for event in events_for_guard {
        match event.event_type {
            EventType::FallsAsleep => {
                sleep_begin = Some(event.timestamp);
            }
            EventType::WakesUp => {
                let duration = event.timestamp - sleep_begin.unwrap();
                let begin = sleep_begin.unwrap().minute();
                let length = duration.num_minutes() as u32;
                for i in 0..length {
                    minutes.push((begin + i) % 60);
                }
                sleep_begin = None;
            }
            EventType::BeginsShift => {}
        }
    }

    let mut minute_counts: Vec<(u32, usize)> = Vec::new();

    for i in 0..60 {
        let count = minutes.iter().filter(|&&x| x == i).count();
        if count > 0 {
            minute_counts.push((i, count));
        }
    }

    minute_counts.sort_by_key(|p| p.1);
    let most_common_minute = minute_counts.last().unwrap().0;

    let part1_result = sleepiest_guard * most_common_minute as i32;
    println!("Day 4 part 1 result is {}", part1_result);

    let mut most_common_minute_for_guard: Option<(i32, u32, usize)> = None;
    // Part 2
    for id in &unique_ids {
        let events_for_guard = events.iter().filter(|event| event.id == *id);

        // Collect all of the minutes the the sleepiest guard spends asleep
        let mut minutes: Vec<u32> = Vec::new();
        let mut sleep_begin: Option<DateTime<Utc>> = None;
        for event in events_for_guard {
            match event.event_type {
                EventType::FallsAsleep => {
                    sleep_begin = Some(event.timestamp);
                }
                EventType::WakesUp => {
                    let duration = event.timestamp - sleep_begin.unwrap();
                    let begin = sleep_begin.unwrap().minute();
                    let length = duration.num_minutes() as u32;
                    for i in 0..length {
                        minutes.push((begin + i) % 60);
                    }
                    sleep_begin = None;
                }
                EventType::BeginsShift => {}
            }
        }

        let mut minute_counts: Vec<(u32, usize)> = Vec::new();

        // Count number of times each minute occurs
        for i in 0..60 {
            let count = minutes.iter().filter(|&&x| x == i).count();
            if count > 0 {
                minute_counts.push((i, count));
            }
        }

        // Find highest minute count for all guards
        minute_counts.sort_by_key(|p| p.1);
        if minute_counts.last() != None {
            let most_common_minute = minute_counts.last().unwrap().0;
            let count = minute_counts.last().unwrap().1;

            if most_common_minute_for_guard == None {
                most_common_minute_for_guard = Some((*id, most_common_minute, count));
            } else if count > most_common_minute_for_guard.unwrap().2 {
                most_common_minute_for_guard = Some((*id, most_common_minute, count));
            }
        }
    }

    let part2_result =
        most_common_minute_for_guard.unwrap().0 * most_common_minute_for_guard.unwrap().1 as i32;

    println!("Day 4 part 2 result is {}", part2_result);
}

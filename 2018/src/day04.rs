use std::collections::HashSet;
use std::fs;

#[derive(PartialEq)]
enum State {
    Started,
    Awake,
    Asleep,
}

struct Event {
    state: State,
    guard: usize,
    beg_sleep: usize,
    end_sleep: usize,
}

pub fn day04() {
    let input = "data/input-04.txt";
    let file = fs::read_to_string(input).unwrap().replace('[', "");
    let mut lines: Vec<_> = file
        .lines()
        .map(|line| line.split("] ").collect::<Vec<_>>())
        .collect();
    lines.sort_unstable();

    let mut events = Vec::<Event>::new();
    let (mut guards, mut current_guard) = (HashSet::new(), 0);
    for (idx, line) in lines.iter().enumerate() {
        let minute = line[0][14..].parse::<usize>().unwrap();
        if (idx > 0) && (events[idx - 1].state == State::Asleep) {
            events[idx - 1].end_sleep = minute;
        }
        let (state, beg_sleep) = match line[1] {
            "falls asleep" => (State::Asleep, minute),
            "wakes up" => (State::Awake, 0),
            other => {
                current_guard = other
                    .replace("Guard #", "")
                    .replace(" begins shift", "")
                    .parse::<usize>()
                    .unwrap();
                (State::Started, 0)
            }
        };
        events.push(Event {
            guard: current_guard,
            state,
            beg_sleep,
            end_sleep: 0, // set at the next iteration if State::Asleep
        });
        guards.insert(current_guard);
    }

    let guards = Vec::from_iter(guards);
    let mut minutes = vec![vec![0; 60]; guards.len()];
    let mut sleep = vec![0; guards.len()];
    for event in &events {
        if event.state == State::Asleep {
            let idx = guards.iter().position(|&z| z == event.guard).unwrap();
            sleep[idx] += event.end_sleep - event.beg_sleep;
            for ii in event.beg_sleep..event.end_sleep {
                minutes[idx][ii] += 1usize;
            }
        }
    }
    let (mut max_minute, mut max_times) = (Vec::new(), Vec::new());
    for minute in &minutes {
        let val = minute
            .iter()
            .enumerate()
            .max_by_key(|(_, &val)| val)
            .unwrap();
        max_minute.push(val.0);
        max_times.push(*val.1);
    }
    let which_max = |vec: &[usize]| {
        vec.iter()
            .enumerate()
            .max_by_key(|(_, &val)| val)
            .map(|(idx, _)| idx)
            .unwrap()
    };

    let guard_id = which_max(&sleep);
    let res = guards[guard_id] * max_minute[guard_id];
    println!("Part 1: {res}");
    assert_eq!(res, 12169);

    let guard_id = which_max(&max_times);
    let res = guards[guard_id] * max_minute[guard_id];
    println!("Part 2: {res}");
    assert_eq!(res, 16164);
}

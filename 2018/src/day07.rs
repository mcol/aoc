use std::collections::HashMap;
use std::fs;

struct Task {
    name: char,
    endtime: usize,
}
struct Workers {
    tasks: Vec<Task>,
    time: usize,
}
impl Workers {
    const MAX: usize = 5;
    fn new() -> Self {
        Workers {
            tasks: Vec::new(),
            time: 0,
        }
    }
    fn add(&mut self, task: char, length: usize) {
        if (self.tasks.len() == Workers::MAX) || self.tasks.iter().any(|z| z.name == task) {
            return;
        }
        self.tasks.push(Task {
            name: task,
            endtime: self.time + length,
        });
    }
    fn ended_tasks(&mut self) -> Vec<char> {
        self.tasks.sort_unstable_by_key(|z| z.endtime);
        self.time = self.tasks[0].endtime;
        match self.tasks.iter().rposition(|z| z.endtime == self.time) {
            Some(pos) => self.tasks.drain(0..=pos).map(|z| z.name).collect(),
            None => Vec::new(),
        }
    }
}
fn remove_task(list: &mut HashMap<char, Vec<Option<char>>>, task: char) {
    list.remove(&task);
    list.iter_mut()
        .for_each(|z| z.1.retain(|&x| x != Some(task)));
}

pub fn day07() {
    let input = "data/input-07.txt";
    let file = fs::read_to_string(input)
        .unwrap()
        .replace("Step ", "")
        .replace(" must be finished before step ", "")
        .replace(" can begin.", "");
    let mut prec = HashMap::new();
    for line in file.lines() {
        if let [bef, aft] = line.chars().collect::<Vec<_>>()[..] {
            prec.entry(bef).or_insert(Vec::new());
            prec.entry(aft).or_insert(Vec::new()).push(Some(bef));
        }
    }

    let (mut prec1, mut res) = (prec.clone(), String::new());
    while let Some((&task, _)) = prec1
        .iter()
        .min_by_key(|&(&k, ref v)| v.len() * 1000 + k as usize)
    {
        remove_task(&mut prec1, task);
        res.push(task);
    }
    println!("Part 1: {res}");
    assert_eq!(res, "BCADPVTJFZNRWXHEKSQLUYGMIO");

    let mut workers = Workers::new();
    while !prec.is_empty() {
        let next_tasks = prec.iter().filter(|(_, v)| v.is_empty()).map(|(k, _)| k);
        for &task in next_tasks {
            workers.add(task, 60 + task as usize - 'A' as usize + 1);
        }
        for task in workers.ended_tasks() {
            remove_task(&mut prec, task);
        }
    }
    let res = workers.time;
    println!("Part 2: {res}");
    assert_eq!(res, 973);
}

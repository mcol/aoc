use std::fs;

fn split_order(patt: &str) -> String {
    let mut chars: Vec<char> = patt.chars().collect();
    chars.sort();
    String::from_iter(chars)
}

fn find_nums(patts: &Vec<Vec<String>>) -> Vec<Vec<&str>> {
    let mut all_nums = Vec::new();
    for row in patts {
        let mut nums = vec![""; 10];
        nums[1] = row
            .iter()
            .find(|z| z.len() == 2)
            .unwrap();
        nums[3] = row
            .iter()
            .find(|z| (z.len() == 5) & nums[1].chars().all(|zz| z.contains(zz)))
            .unwrap();
        nums[4] = row
            .iter()
            .find(|z| z.len() == 4)
            .unwrap();
        nums[7] = row
            .iter()
            .find(|z| z.len() == 3)
            .unwrap();
        nums[8] = row
            .iter()
            .find(|z| z.len() == 7)
            .unwrap();
        nums[9] = row
            .iter()
            .find(|z| (z.len() == 6) & nums[4].chars().all(|zz| z.contains(zz)))
            .unwrap();
        nums[0] = row
            .iter()
            .find(|z| (z.len() == 6) & (z != &nums[9]) & nums[1].chars().all(|zz| z.contains(zz)))
            .unwrap();
        nums[5] = row
            .iter()
            .find(|z| (z.len() == 5) & (z != &nums[3]) & z.chars().all(|zz| nums[9].contains(zz)))
            .unwrap();
        nums[6] = row
            .iter()
            .find(|z| (z.len() == 6) & (z != &nums[0]) & (z != &nums[9]))
            .unwrap();
        nums[2] = row
            .iter()
            .find(|z| (z.len() == 5) & (z != &nums[3]) & (z != &nums[5]))
            .unwrap();
        all_nums.push(nums);
    }
    all_nums
}

fn decode(nums: &Vec<&str>, codes: &[String]) -> i32 {
    let (mut sum, len_codes) = (0, codes.len());
    for (idx_code, code) in codes.iter().enumerate() {
        for (idx_num, &val) in nums.iter().enumerate() {
            if code == val {
                sum += idx_num as i32 * i32::pow(10, (len_codes - 1 - idx_code) as u32);
            }
        }
    }
    sum
}

pub fn day08() {
    let input = "data/input-08.txt";
    let file = fs::read_to_string(input).unwrap();

    let (mut patts, mut codes) = (Vec::new(), Vec::new());
    for line in file.lines() {
        let mut split = line.split(" | ");

        let vals: Vec<String> = split
            .next()
            .expect("")
            .split_whitespace()
            .map(split_order)
            .collect();
        patts.push(vals);

        let vals: Vec<String> = split
            .next()
            .expect("")
            .split_whitespace()
            .map(split_order)
            .collect();
        codes.push(vals);
    }

    let mut res: usize = 0;
    for val in &codes {
        res += val
            .iter()
            .map(|z| z.len())
            .filter(|z| (*z == 2) | (*z == 3) | (*z == 4) | (*z == 7))
            .count();
    }
    println!("Part 1: {}", res);

    let nums = find_nums(&patts);
    let res: i32 = (0..nums.len()).map(|z| decode(&nums[z], &codes[z])).sum();
    println!("Part 2: {}", res);
}

fn count_permutation(shipments: &Vec<u32>) -> Option<(usize, Vec<String>)> {
    if shipments.is_empty() {
        return Some((0, vec![]));
    }

    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return None;
    }

    let target = total / n;
    let mut moves = 0;
    let mut steps = vec![];

    let mut current = shipments.clone();
    for i in 0..current.len() {
        while current[i] != target {
            for j in 0..current.len() {
                if i == j {
                    continue;
                }
                if current[i] > target && current[j] < target {
                    let transfer = (current[i] - target).min(target - current[j]);
                    current[i] -= transfer;
                    current[j] += transfer;
                    moves += transfer as usize;
                    let mut step = vec!["".to_string(); shipments.len()];
                    step[i] = format!("-{}", transfer);
                    step[j] = format!("+{}", transfer);
                    steps.push(step.join(" "));
                }
                else if current[i] < target && current[j] > target {
                    let transfer = (target - current[i]).min(current[j] - target);
                    current[i] += transfer;
                    current[j] -= transfer;
                    moves += transfer as usize;
                    let mut step = vec!["".to_string(); shipments.len()];
                    step[i] = format!("+{}", transfer);
                    step[j] = format!("-{}", transfer);
                    steps.push(step.join(" "));
                }
            }
        }
    }

    Some((moves, steps))
}

fn print_solution(shipments: &Vec<u32>) {
    println!("{:?}", shipments);

    let total: u32 = shipments.iter().sum();
    let n = shipments.len();
    println!("total   = {}", shipments.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" + "));
    println!("average = {} / {} = {}", total, n, total / n as u32);

    if let Some((moves, steps)) = count_permutation(shipments) {
        println!();
        println!("{:?}", shipments);
        for step in steps {
            println!("{}", step);
        }
        println!();
        println!("answer = {}", moves);
    } else {
        println!("Cannot distribute equally");
    }
    println!("—----—----—----—----—----—----—----—----—----—--");
}

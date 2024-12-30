const INPUT: &str = r#""#;

fn main() {
    println!("Hello, world!");

    let (mut left, mut right) = create_vecs(INPUT);

    left.sort();
    right.sort();

    dbg!(calculate_total_distance(&left, &right));
    dbg!(calculate_similarity(&left, &right));
}

fn create_vecs(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();

            dbg!(left);
            dbg!(right);
            (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
        })
        .collect()
}

fn calculate_total_distance(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    left.iter().zip(right)
        .fold(0, |total, (l, r)| {
            total + (l - r).abs()
        })

}

fn calculate_similarity(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in left {
        let mut count_in_right = 0;
        let mut found = false;

        for j in right {
            if found == false {
                if i == j {
                    found = true;
                }
                else {
                    continue;
                }
            }

            if i == j {
                count_in_right += 1;
            }
            else {
                break;
            }

        }

        sum += count_in_right * i;

    }

    sum

}

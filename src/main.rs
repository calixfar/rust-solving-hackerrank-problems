use std::{io, vec};

fn main() {
    no_divisible_subset();
}

// https://www.hackerrank.com/challenges/non-divisible-subset/problem?isFullScreen=true
fn no_divisible_subset() {
    let vector: Vec<usize> = vec![1,7,2,4];
    let multiple: usize = 3;
    let mut no_divisible_numbers = Vec::new();
    let mut index: usize = 0;
    let mut sub_index: usize = 0;


    while index < vector.len() {
        while sub_index < vector.len() {
            let next_index = sub_index + 1;

            if next_index < vector.len() {
                let sum = vector[index] + vector[next_index];
                if sum % multiple != 0 {
                    if !is_element_in_array(no_divisible_numbers.clone(), vector[index]) {
                        no_divisible_numbers.push(vector[index]);
                    }

                    if !is_element_in_array(no_divisible_numbers.clone(), vector[next_index]) {
                        no_divisible_numbers.push(vector[next_index]);
                    }
                }
            }
            sub_index += 1;
        }

        index += 1;
        sub_index = index;
    }

    println!("{:?}", no_divisible_numbers);
}

// https://www.hackerrank.com/challenges/extra-long-factorials/problem?isFullScreen=true
fn extra_long_factorials_loop() {
    extra_long_factorials();

    loop {
        let mut line = String::new();
        
        println!("Do you want to get another factorial? y/n");
    
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        
        line = line.trim().to_string();

        if line == "y" {
            extra_long_factorials();
        } else {
            break;
        }
    }   
}

fn extra_long_factorials() {
    let mut line = String::new();

    println!("Please type the number: ");

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    let mut number: i128 = line
        .trim()
        .parse::<i128>()
        .expect("Error");
    
    let mut value = 1;

    while number > 1 {
        value *= number;
        
        number -= 1;
    }
    

    println!("This is the value: {}", value);
}

// Problem: https://www.hackerrank.com/challenges/climbing-the-leaderboard/problem?isFullScreen=true
fn climbing_leader_board() -> Vec<usize> {
    let ranked = vec![100, 100, 50, 40, 40, 20, 10];
    let player = vec![5, 25, 50, 120];
    let mut new_positions = Vec::new();

    for player_item in player {
        let new_position = get_position(ranked.clone(), player_item);
        
        new_positions.push(new_position);
    }
    println!("{:?}", new_positions);


    new_positions
}

fn get_position(vector: Vec<usize>, item_to_find: usize) -> usize {
    let normalized_vector = normalize_vector(vector);
    let mut index = 0;
    
    while index < normalized_vector.len() {
        if normalized_vector[index] == item_to_find || normalized_vector[index] < item_to_find {
            return index + 1;
        } 

        index += 1;
    }

    normalized_vector.len() + 1
}

fn is_element_in_array(vector: Vec<usize>, item_to_find: usize) -> bool {
    for item in vector {
        if item == item_to_find {

            return true;
        }
    }

    return false;
}

fn normalize_vector(vector: Vec<usize>) -> Vec<usize> {
    let mut normalized_vector = Vec::new();

    for item in vector {
        if !is_element_in_array(normalized_vector.clone(), item) {
            normalized_vector.push(item);
        }
    }

    normalized_vector
}

fn a_very_big_sum() -> usize {
    let mut nums: Vec<usize> = Vec::new();

    nums.push(1);
    nums.push(2);
    nums.push(3);
    nums.push(4);
    nums.push(5);
    nums.push(6);

    let mut sum = 0;

    for num in nums {
        sum += num
    }

    println!("The result is: {}", sum);

    sum
}

fn compare_triplets_inline_solution() -> [i8; 2] {
    let first_jury_points: [i8; 3] = [5, 7, 11];
    let second_jury_points: [i8; 3] = [3, 6, 10];

    const BREAK_POINT: usize = 3;
    let mut step: usize = 0;
    let mut first_jury_score = 0;
    let mut second_jury_score = 0;
    let mut break_point_multiplier = 1;

    let juries_score: [i8; 2] = [
        loop {
            if step == BREAK_POINT {
                break_point_multiplier += 1;

                break first_jury_score;
            }

            if first_jury_points[step] > second_jury_points[step] {
                first_jury_score += 1;
            }

            step += 1;
        },
        loop {
            if step == BREAK_POINT * break_point_multiplier {
                break second_jury_score;
            }

            let index: usize = step % 3;

            if second_jury_points[index] > first_jury_points[index] {
                second_jury_score += 1;
            }

            step += 1;
        }
    ];

    print_result(juries_score);

    juries_score
}

fn simple_compare_triplets(first_jury_points: [i8; 3], second_jury_points: [i8; 3]) -> [i8; 2] {
    let mut juries_score: [i8; 2] = [0, 0];
    let mut step: usize = 0;

    loop {
        if step == 3 {
            break;
        }

        if first_jury_points[step] > second_jury_points[step] {
            juries_score[0] += 1;
        } else if first_jury_points[step] < second_jury_points[step] {
            juries_score[1] += 1;
        }

        step += 1;
    }

    print_result(juries_score);
    
    juries_score
}

fn print_result (scores: [i8; 2]) {
    println!("=======Result=======");
    println!("First jury' score: {}", scores[0]);
    println!("Second jury' score: {}", scores[1]);
}
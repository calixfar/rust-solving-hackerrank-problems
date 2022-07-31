fn main() {
    let first_jury_points: [i8; 3] = [5, 7, 11];
    let second_jury_points: [i8; 3] = [3, 6, 10];

    compare_triplets_inline_solution(first_jury_points, second_jury_points);
}

fn compare_triplets_inline_solution(first_jury_points: [i8; 3], second_jury_points: [i8; 3]) -> [i8; 2] {
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
fn main() {
    let first_jury_points: [i8; 3] = [5, 6, 7];
    let second_jury_points: [i8; 3] = [3, 6, 10];

    simple_compare_triplets(first_jury_points, second_jury_points);
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

    println!("=======Result=======");
    println!("First jury' score: {}", juries_score[0]);
    println!("Second jury' score: {}", juries_score[1]);

    
    juries_score
}


fn convert_string_to_tuple(input: String) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("unable to convert String to i32"))
        .collect()
}

fn compare_scores(scores_1: Vec<i32>, scores_2: Vec<i32>) -> (i32, i32) {
    let mut total_score_1 = 0;
    let mut total_score_2 = 0;
    
    for (a, b) in scores_1.iter().zip(&scores_2) {
        if a > b {
            total_score_1 += 1;
        } else if a < b {
            total_score_2 += 1;
        }
    }

    return (total_score_1, total_score_2);
}

pub fn compare_scores_strings(scores_1: String, scores_2: String) -> (i32, i32) {
    return compare_scores(
        convert_string_to_tuple(scores_1),
        convert_string_to_tuple(scores_2)
    );
}



#[cfg(test)]
mod test {

    use super::compare_scores_strings;

    #[test]
    fn name() {
        let scores_1 = "5 6 7".to_string();
        let scores_2 = "3 6 10".to_string();
        let expected_results = (1, 1);
        let actual_results = compare_scores_strings(scores_1, scores_2);
        assert_eq!(expected_results, actual_results);
    }
}

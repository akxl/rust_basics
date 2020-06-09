
use std::collections::HashMap;

fn build_ranking_table(scores: Vec<i32>) -> HashMap<i32, i32> {
    
    let mut _scores = scores.clone();
    _scores.sort_by(|a, b| b.cmp(a));
    _scores.dedup();

    let mut table: HashMap<i32, i32> = HashMap::new();
    for (index, score) in _scores.iter().enumerate() {
        if !table.contains_key(score) {
            table.insert(*score, index as i32 + 1);
        }
    }

    return table;

}

fn find_rank(existing_scores: Vec<i32>, dudes_score:  i32) -> i32 {

    let mut _scores = existing_scores.clone();
    _scores.push(dudes_score);
    let ranking_table = build_ranking_table(_scores);
    return *ranking_table.get(&dudes_score).unwrap();

}

#[cfg(test)]
mod test {

    #[test]
    fn name() {

        // Sample 1
        //7
        //100 100 50 40 40 20 10
        //4
        //5 25 50 120
    
        //6
        //4
        //2
        //1

        assert_eq!(6, super::find_rank(vec![100, 100, 50, 40, 40, 20, 10], 5));
        assert_eq!(4, super::find_rank(vec![100, 100, 50, 40, 40, 20, 10], 25));
        assert_eq!(2, super::find_rank(vec![100, 100, 50, 40, 40, 20, 10], 50));
        assert_eq!(1, super::find_rank(vec![100, 100, 50, 40, 40, 20, 10], 120));

        // Sample 2
        //6
        //100 90 90 80 75 60
        //5
        //50 65 77 90 102

        //6
        //5
        //4
        //2
        //1
        assert_eq!(6, super::find_rank(vec![100, 90, 90, 80, 75, 60], 50));
        assert_eq!(5, super::find_rank(vec![100, 90, 90, 80, 75, 60], 65));
        assert_eq!(4, super::find_rank(vec![100, 90, 90, 80, 75, 60], 77));
        assert_eq!(2, super::find_rank(vec![100, 90, 90, 80, 75, 60], 90));
        assert_eq!(1, super::find_rank(vec![100, 90, 90, 80, 75, 60], 102));

    }
    
    

    
}
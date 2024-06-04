use std::cmp::min;

#[derive(Clone, Copy)]
pub struct FuzzyConfig {
    threshold: usize,
    insertion_penalty: Option<usize>,
    deletion_penalty: Option<usize>,
    substitution_penalty: Option<usize>,
}

pub fn fuzzy_match<'a, Value>(needle: &'a String, haystack: &'a Vec<(String, &'a Value)>, config: FuzzyConfig) -> Option<&'a(String, &'a Value)> {
    let mut res = None;
    let mut threshold = config.threshold;
    for hay in haystack {
        let check_res = check(needle, &hay.0, FuzzyConfig { threshold, ..config });
        if check_res.is_some() {
            let check_res = check_res.unwrap();
            threshold = check_res.1;
            res = Some(hay);
        }
    }
    res
}

fn check<'a>(needle: &'a String, candidate: &'a String, config: FuzzyConfig) -> Option<(&'a String, usize)> {
    if needle.len() == 0 || candidate.len() == 0 {
        return None;
    }

    let FuzzyConfig {
        threshold,
        insertion_penalty,
        deletion_penalty,
        substitution_penalty,
    } = config;
    let insertion_penalty = insertion_penalty.unwrap_or(1);
    let deletion_penalty = deletion_penalty.unwrap_or(1);
    let substitution_penalty = substitution_penalty.unwrap_or(2);
    
    let mut prev_row = vec![];
    prev_row.push(0);
    for i in 1..=needle.len() {
        prev_row[i] = prev_row[i - 1] + deletion_penalty;
    }

    let mut cur_row = vec![];
    
    for candidate_c in candidate.chars() {
        cur_row.push(prev_row[0] + insertion_penalty);
        for (needle_i, needle_c) in needle.chars().enumerate() {
            let mut min_cost = std::usize::MAX;
            if needle_c == candidate_c {
                min_cost = *prev_row.get(needle_i).unwrap_or(&std::usize::MAX);
            } else {
                min_cost = *cur_row.get(needle_i).unwrap_or(&std::usize::MAX) + deletion_penalty;
                min_cost = min(min_cost, *prev_row.get(needle_i + 1).unwrap_or(&std::usize::MAX) + insertion_penalty);
                min_cost = min(min_cost, *prev_row.get(needle_i).unwrap_or(&std::usize::MAX) + substitution_penalty);
            }
            if min_cost > 0 {
                break;
            }
            cur_row.push(min_cost);
        }
        prev_row = cur_row;
        cur_row = vec![];
    }

    prev_row.last()
            .and_then(|i| if *i <= threshold { Some((candidate, *i)) } else { None })
}

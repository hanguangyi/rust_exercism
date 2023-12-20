use std::collections::{HashSet, HashMap};
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut return_set: HashSet<&str> = HashSet::new();
    let word_vec: Vec<char> = word.to_lowercase().chars().collect();
    let mut word_map: HashMap<char, i32> = HashMap::new();
    for i in word_vec{
        let get_char: Option<&i32> = word_map.get(&i);
        let get_char: i32 =    match get_char {
                Some(c) => {
                    let count: i32 = c+1;
                    count
                },
                None => {
                    1
                }
            };
        word_map.insert(i, get_char);       
    }

    for i in possible_anagrams{
        if the_same(word, i){
            continue;
        }else if word.len() != i.len() {
            println!("i is {}  ",i);
            continue;
        }else{
            let param_vec: Vec<char> = i.to_lowercase().chars().collect();
            let mut is_anagram: bool = true;
            let mut word_map_copy = word_map.clone();
            for p in param_vec {
                let get_char: Option<&i32> = word_map_copy.get(&p);
                let get_char = match get_char {
                    Some(c) => {
                        if c == &0{
                            is_anagram =false;
                            break;
                        };
                        c
                    },
                    None => &-1,
                };                
                if get_char.eq(&-1) == true{
                    is_anagram = false;
                    break;
                }else{
                    word_map_copy.insert(p, get_char-1);
                } 
            }
            if is_anagram{
                return_set.insert(i);
            }
        }
    }
    return_set
}


fn the_same(word: &str,param: &str) -> bool{
    // word.to_ascii_lowercase().eq_ignore_ascii_case(& param.to_ascii_lowercase())//can't cover unicode.
    word.to_lowercase().eq(&param.to_lowercase())
}



// another fluent way to solve

// use std::collections::HashSet;

// pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
//     let word_lower = word.to_lowercase();
//     let word_sorted = get_sorted(&word_lower);
//     possible_anagrams
//         .iter()
//         .filter(|candidate| {
//             let candidate_lower = candidate.to_lowercase();
//             candidate_lower.len() == word_lower.len()
//                 && candidate_lower != word_lower
//                 && get_sorted(&candidate_lower) == word_sorted
//         }).copied()
//         .collect()
// }

// fn get_sorted(word: &str) -> Vec<char> {
//     let mut word_sorted: Vec<char> = word.chars().collect();
//     word_sorted.sort_unstable();
//     word_sorted
// }

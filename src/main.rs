fn remove_duplicates<T: Eq + std::hash::Hash + Clone>(vec: &Vec<T>) -> Vec<T> {
    let mut set = std::collections::HashSet::new();
    let mut result = Vec::new();
    for item in vec.iter() {
        if set.insert(item.clone()) {
            result.push(item.clone());
        }
    }
    result
}

fn main() {
    let vec1 = vec![1, 2, 3, 2, 4, 5, 1];
    let vec2 = vec!["a", "b", "c", "b", "d", "e", "a"];

    let unique_vec1 = remove_duplicates(&vec1);
    let unique_vec2 = remove_duplicates(&vec2);

    println!("{:?}", unique_vec1); // [1, 2, 3, 4, 5]
    println!("{:?}", unique_vec2); // ["a", "b", "c", "d", "e"]
}
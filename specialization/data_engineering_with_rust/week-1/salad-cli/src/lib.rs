use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let fruits = vec![
        "Arbutus".to_string(),
        "Loquat".to_string(),
        "Strawberry Tree Berry".to_string(),
        "Pomegranate".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Orange".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
        "Apple".to_string(),
    ];

    // Error if the user requests more fruits than available
    if num_fruits > fruits.len() {
        panic!(
            "{:?} is too big. Maximum number of fruits is {:?}",
            num_fruits,
            fruits.len()
        );
    } else {
        let mut rng = thread_rng();
        let mut fruits = fruits;
        fruits.shuffle(&mut rng);
        fruits.into_iter().take(num_fruits).collect().sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()))
    }
}

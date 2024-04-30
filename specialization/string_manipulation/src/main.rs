// create a function to count words length and get biggest word(s)
fn longest_word(words: Vec<&str>) { // takes a vector of string slices as input
    let mut longest: Vec<String> = vec![""]; // create an empty string variable to store the longest 
    for word in words {
        if word.len() > longest[0].len() {
            longest = vec![word];
        } else if {
            
        } else {continue}
    }
    println!("{longest}");
    println!("The longest word or words is/are: ", words[])

}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    // iterate over the characters in the sentence
    let mut vowels: i32 = 0;
    let mut a_count: i32 = 0;
    let mut e_count: i32 = 0;
    let mut i_count: i32 = 0;
    let mut o_count: i32 = 0;
    let mut u_count: i32 = 0;

    for c in sentence.chars() {
         match c {
             'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
             _ => continue,
         }

         match c {
            'a' => a_count += 1,
            'e' => e_count += 1,
            'i' => i_count += 1,
            'o' => o_count += 1,
            'u' => u_count += 1,
            _ => continue,
        }
    }

    println!("Total count of volwels is {}.", vowels);
    println!("Total count of 'a' is {}.", a_count);
    println!("Total count of 'e' is {}.", e_count);
    println!("Total count of 'i' is {}.", i_count);
    println!("Total count of 'o' is {}.", o_count);
    println!("Total count of 'u' is {}.", u_count);


    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    //println!("{:?}", words);
    longest_word(words);


    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);


}

pub fn print(word: &String, contained: Vec<bool>, correct: Vec<bool>) -> () {
    let mut output: String = String::new();

    for (contained_check, correct_check) in contained.iter().zip(correct.iter()) {
        match (contained_check, correct_check) {
            (true, true) => output.push('$'),
            (true, false) => output.push('#'),
            _ => output.push('_'),
        }
    }

    println!("{}", output);
    println!("{}", word);
}

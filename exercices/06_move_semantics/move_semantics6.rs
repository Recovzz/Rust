// J'ai modifié la fonction get_char pour qu'elle prenne en ref la string j'ai fais pareil avec la fonction string_uppercase. 
// J'ai aussi retiré le mut de data car on en a pas besoin puisque nous ne modifions pas la ref elle même mais + la valeur qu'elle pointe

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) {
    let uppercase_data = data.to_uppercase();

    println!("{}", uppercase_data);
}

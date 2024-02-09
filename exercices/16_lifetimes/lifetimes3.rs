// J'ai ajouté une annotation de durée de vie ('a) à la structure pour indiquer que les références &str contenues dans la structure doivent vivre au moins aussi longtemps que la référence de durée de vie 'a.

struct Book<'a> {
    author: &'a str,  // Référence à une chaîne de caractères (str) avec la durée de vie 'a
    title: &'a str,   // Référence à une chaîne de caractères (str) avec la durée de vie 'a
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    
    // J'ai crée une instance de Book en utilisant les références (&) des chaînes de caractères pour indiquer la durée de vie 
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}

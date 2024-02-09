// Le code original contient une fonction average qui prend en entrée un tableau de f64 et renvoie la moyenne de ces nombres. 
// Cette fonction calcule d'abord la somme de tous les éléments du tableau en utilisant la méthode sum(), puis elle divise cette somme par la longueur du tableau. 
// Cette division est la source d'une erreur de compilation, car la division de deux types différents (f64 et usize) n'est pas autorisée en Rust.
// Pour corriger cette erreur, il est nécessaire de convertir explicitement le type usize en f64 en utilisant le mot-clé as. 
// Cette conversion permet de diviser deux f64 et de retourner un f64, qui est le type attendu pour la fonction average. 
// Le code modifié contient donc un commentaire expliquant cette modification, ainsi qu'un autre commentaire expliquant le calcul de la somme.
// Le module tests contient également un test qui vérifie que la fonction average renvoie la moyenne attendue. Ce test permet de s'assurer que la fonction est correctement implémentée.

fn average(values: &[f64]) -> f64 {
    // La somme des éléments de `values` est une `f64`.
    let total = values.iter().sum::<f64>();

    // Pour diviser la somme par la longueur de `values`, il est nécessaire de
    // convertir cette dernière en un `f64`, car la division de deux `f64` doit
    // retourner un `f64`.
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        // Vérifie que la fonction `average` renvoie la moyenne attendue.
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}

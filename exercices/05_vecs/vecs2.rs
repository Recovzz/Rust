// vec_loop :
// La fonction vec_loop prend un vecteur v en tant que paramètre, itère sur chaque élément de manière mutable à l'aide de iter_mut(), et multiplie chaque élément par 2 en utilisant l'opérateur *.
// Aucune nouvelle allocation n'est effectuée car la modification est faite directement sur le vecteur d'origine. Le vecteur modifié est renvoyé à la fin de la fonction.

// vec_map :
// La fonction vec_map prend une référence à un vecteur v en tant que paramètre, itère sur chaque élément à l'aide de iter(), multiplie chaque élément par 2 en utilisant une fonction de clôture dans map, 
// et collecte les résultats dans un nouveau vecteur à l'aide de collect().
// Contrairement à vec_loop, cette approche crée un nouveau vecteur pour stocker les résultats des opérations de multiplication. 
// Cela permet de ne pas modifier le vecteur d'origine, ce qui peut être utile dans certains cas.

// Cette fonction prend un vecteur d'entiers `v`, itère sur chaque élément de manière mutable, et multiplie chaque élément par 2.
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        *element *= 2; // Multiplie chaque élément par 2
    }
    v 
}

// Cette fonction prend une référence à un vecteur d'entiers `v`, itère sur chaque élément, multiplie chaque élément par 2, et collecte les résultats dans un nouveau vecteur.
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| *element * 2).collect() // Multiplie chaque élément par 2 et collecte dans un nouveau Vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        // Crée un vecteur `v` contenant les 5 premiers nombres pairs (2, 4, 6, 8, 10)
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        // Appelle vec_loop pour multiplier chaque élément par 2
        let ans = vec_loop(v.clone());

        // Vérifie si le vecteur résultant `ans` est égal au vecteur attendu (4, 8, 12, 16, 20)
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        // Crée un vecteur `v` contenant les 5 premiers nombres pairs (2, 4, 6, 8, 10)
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        // Appelle vec_map pour multiplier chaque élément par 2
        let ans = vec_map(&v);

        // Vérifie si le vecteur résultant `ans` est égal au vecteur attendu (4, 8, 12, 16, 20)
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}

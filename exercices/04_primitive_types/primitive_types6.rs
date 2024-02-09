// J'ai remplacé les ??? par numbers.1; car numbers est déclaré dans la fonction l'indexation des tuples commençent par 0 donc numbers.1 renvoi au 2e élément du tuple

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}

// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Implémenter une méthode new pour créer une nouvelle instance de Rectangle
    pub fn new(width: i32, height: i32) -> Self {
        // Vérifier si les dimensions sont positives
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // Créer une instance de Rectangle avec des dimensions spécifiques
        let rect = Rectangle::new(10, 20);
        // Vérifier si la largeur est correcte en utilisant l'assertion assert_eq!
        assert_eq!(rect.width, 10);
        // Vérifier si la hauteur est correcte en utilisant l'assertion assert_eq!
        assert_eq!(rect.height, 20);
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        // Vérifier si le programme panique lors de la création d'un Rectangle avec une largeur négative
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        // Vérifier si le programme panique lors de la création d'un Rectangle avec une hauteur négative
        let _rect = Rectangle::new(10, -10);
    }
}

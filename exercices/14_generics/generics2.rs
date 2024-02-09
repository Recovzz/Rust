// 

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        // Pour envelopper une chaîne de caractères, nous devons spécifier le type explicite
        // car Rust ne peut pas inférer le type pour nous.
        assert_eq!(Wrapper::new("Foo".to_string()).value, "Foo");
    }
}

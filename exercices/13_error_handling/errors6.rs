
use std::num::ParseIntError;

// C'est un type d'erreur personnalisé dans `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),        // Erreur lors de la création de PositiveNonzeroInteger
    ParseInt(ParseIntError),        // Erreur lors du parsing d'un entier
}

impl ParsePosNonzeroError {
    // Fonction de conversion pour CreationError en ParsePosNonzeroError
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }

    // Fonction de conversion pour ParseIntError en ParsePosNonzeroError
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

// Fonction pour parser un entier positif non nul à partir d'une chaîne
fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;  // Parser la chaîne en un entier
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)  // Tenter de créer PositiveNonzeroInteger à partir de l'entier parsé
}

// Erreur personnalisée pour représenter les erreurs pouvant survenir lors de la création de PositiveNonzeroInteger
#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,   // Erreur : Valeur négative
    Zero,       // Erreur : Valeur zéro
}

// Structure pour représenter un entier positif non nul
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    // Fonction pour créer un PositiveNonzeroInteger à partir d'une valeur i64
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),   // La valeur négative n'est pas autorisée
            0 => Err(CreationError::Zero),               // La valeur zéro n'est pas autorisée
            x => Ok(PositiveNonzeroInteger(x as u64)),   // Créer PositiveNonzeroInteger à partir de la valeur non négative
        }
    }
}

// Tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}

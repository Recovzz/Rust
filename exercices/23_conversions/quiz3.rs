// Je vais utiliser une énumération pour représenter les notes alphabétiques et numériques.
// Ensuite, je vais implémenter un trait qui convertit les notes numériques en notes alphabétiques et vice versa.

pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

// J'implémente le trait GradeConverter qui aura une méthode convert_grade.
trait GradeConverter {
    fn convert_grade(&self) -> String;
}

// J'implémente le trait pour les types f32 et String pour convertir les grades numériques en grades alphabétiques et vice versa.
impl GradeConverter for f32 {
    fn convert_grade(&self) -> String {
        if *self >= 4.5 {
            "A+".to_string()
        } else if *self >= 4.0 {
            "A".to_string()
        } else if *self >= 3.5 {
            "B+".to_string()
        } else if *self >= 3.0 {
            "B".to_string()
        } else if *self >= 2.5 {
            "C+".to_string()
        } else if *self >= 2.0 {
            "C".to_string()
        } else if *self >= 1.5 {
            "D+".to_string()
        } else if *self >= 1.0 {
            "D".to_string()
        } else {
            "F".to_string()
        }
    }
}

impl GradeConverter for String {
    fn convert_grade(&self) -> String {
        match self.as_str() {
            "A+" => "4.5".to_string(),
            "A" => "4.0".to_string(),
            "B+" => "3.5".to_string(),
            "B" => "3.0".to_string(),
            "C+" => "2.5".to_string(),
            "C" => "2.0".to_string(),
            "D+" => "1.5".to_string(),
            "D" => "1.0".to_string(),
            _ => "0.0".to_string(),
        }
    }
}

// Implémentation de la méthode print pour ReportCard en utilisant le trait GradeConverter.
impl<T: GradeConverter> ReportCard<T> {
    pub fn print(&self) -> String {
        let grade = self.grade.convert_grade();
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, grade
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // Je passe une chaîne de caractères représentant la note alphabétique.
        let report_card = ReportCard {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}

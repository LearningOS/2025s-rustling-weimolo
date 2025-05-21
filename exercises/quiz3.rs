// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.

// ai

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    // Convert numeric grade to letter grade
    fn grade_to_letter(grade: f32) -> String {
        if grade >= 4.0 {
            "A+".to_string()  // Example thresholds, adjust as needed
        } else if grade >= 3.0 {
            "A".to_string()
        } else if grade >= 2.0 {
            "B".to_string()
        } else if grade >= 1.0 {
            "C".to_string()
        } else {
            "F".to_string()
        }
    }

    // Print the report card with the letter grade
    pub fn print(&self) -> String {
        let letter_grade = ReportCard::grade_to_letter(self.grade);
        format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, letter_grade)
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
            "Tom Wriggle (12) - achieved a grade of B"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: 4.5,  // Change the grade here to test letter grade
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}

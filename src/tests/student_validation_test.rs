#[cfg(test)]
mod tests {
    use crate::models::CreateStudent;
    use chrono::{NaiveDate, Utc, Duration};

    // Helper function to create a valid student for tests
    fn valid_student_data() -> CreateStudent {
        CreateStudent {
            name: "John".to_string(),
            surname: "Doe".to_string(),
            phonenumber: "123-456-7890".to_string(),
            birthday: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            email: "john.doe@example.com".to_string(),
        }
    }

    #[test]
    fn test_valid_student() {
        let student = valid_student_data();
        assert!(student.validate().is_ok());
    }

    #[test]
    fn test_empty_name() {
        let mut student = valid_student_data();
        student.name = "".to_string();
        let errors = student.validate().unwrap_err();
        assert!(errors.contains(&"Name cannot be empty.".to_string()));
    }

    #[test]
    fn test_empty_surname() {
        let mut student = valid_student_data();
        student.surname = "".to_string();
        let errors = student.validate().unwrap_err();
        assert!(errors.contains(&"Surname cannot be empty.".to_string()));
    }

    #[test]
    fn test_empty_phonenumber() {
        let mut student = valid_student_data();
        student.phonenumber = "".to_string();
        let errors = student.validate().unwrap_err();
        assert!(errors.contains(&"Phone number cannot be empty.".to_string()));
    }

    #[test]
    fn test_empty_email() {
        let mut student = valid_student_data();
        student.email = "".to_string();
        let errors = student.validate().unwrap_err();
        assert!(errors.contains(&"Email cannot be empty.".to_string()));
    }

    #[test]
    fn test_invalid_email_formats() {
        let invalid_emails = vec!["test@test", "test.com", "test@.com", "@example.com", "test@domain."];
        for email in invalid_emails {
            let mut student = valid_student_data();
            student.email = email.to_string();
            let errors = student.validate().unwrap_err();
            assert!(errors.contains(&"Invalid email format.".to_string()), "Failed for email: {}", email);
        }
    }

    #[test]
    fn test_valid_email_formats() {
        let valid_emails = vec!["test@example.com", "test.name@example.co.uk", "user123@sub.domain.info"];
        for email in valid_emails {
            let mut student = valid_student_data();
            student.email = email.to_string();
            assert!(student.validate().is_ok(), "Failed for email: {}", email);
        }
    }

    #[test]
    fn test_invalid_phone_number_formats() {
        let invalid_phones = vec!["12345", "123-456-789XX", "12345678901", "(123)456-7890", "123 456 789"]; // Last one needs 10 digits
        for phone in invalid_phones {
            let mut student = valid_student_data();
            student.phonenumber = phone.to_string();
            let errors = student.validate().unwrap_err();
            assert!(errors.contains(&"Invalid phone number format. Expected XXX-XXX-XXXX or (XXX) XXX-XXXX.".to_string()), "Failed for phone: {}", phone);
        }
    }

    #[test]
    fn test_valid_phone_number_formats() {
        let valid_phones = vec!["123-456-7890", "(123) 456-7890", "123 456 7890", "(123)456-7890", "123-456-7890"];
        for phone in valid_phones {
            let mut student = valid_student_data();
            student.phonenumber = phone.to_string();
            assert!(student.validate().is_ok(), "Failed for phone: {}", phone);
        }
    }

    #[test]
    fn test_birthday_in_future() {
        let mut student = valid_student_data();
        student.birthday = Utc::now().date_naive() + Duration::days(1);
        let errors = student.validate().unwrap_err();
        assert!(errors.contains(&"Birthday cannot be in the future.".to_string()));
    }

    #[test]
    fn test_birthday_today() {
        let mut student = valid_student_data();
        student.birthday = Utc::now().date_naive();
        assert!(student.validate().is_ok());
    }

    #[test]
    fn test_birthday_in_past() {
        let mut student = valid_student_data();
        student.birthday = Utc::now().date_naive() - Duration::days(1);
        assert!(student.validate().is_ok());
    }
}

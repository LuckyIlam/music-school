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
        let invalid_phones = vec![
            "123456789",         // missing leading 0 or country code
            "012345678",         // too short
            "+33 1 23 45 67 8",  // too short with country code
            "0044123456789",     // wrong country code
            "01-23-45-67-890",   // too long
            "012-345-6789",      // incorrect grouping for French numbers
            "+33 (0) 1 23 45 67 89 01", // too long with (0)
            "01.23.45.67.89.",   // trailing separator
            ".01.23.45.67.89",  // leading separator
            "01234567890",       // too long, no separators
            "+331234567890",     // too long, with +33
            "00331234567890",    // too long, with 0033
            "0 1 2 3 4 5 6 7 8 9", // separators between each digit
            "012345678A",        // invalid character
        ];
        for phone in invalid_phones {
            let mut student = valid_student_data();
            student.phonenumber = phone.to_string();
            let result = student.validate();
            assert!(result.is_err(), "Expected error for phone: {}", phone);
            if let Err(errors) = result {
                assert!(errors.contains(&"Invalid phone number format.".to_string()), "Failed for phone: {}. Errors: {:?}", phone, errors);
            }
        }
    }

    #[test]
    fn test_valid_phone_number_formats() {
        let valid_phones = vec![
            "0123456789",
            "+33123456789",
            "0033123456789",
            "01 23 45 67 89",
            "+33 1 23 45 67 89",
            "0033 (0)1 23 45 67 89", // Handled by (?:\(0\)[ ]?)?
            "06.12.34.56.78",
            "0033(0)123456789",      // Valid: No space after (0)
            "+33 (0) 123456789",     // Valid: Space after (0)
            "01-23-45-67-89",        // Valid: Hyphenated
        ];
        for phone in valid_phones {
            let mut student = valid_student_data();
            student.phonenumber = phone.to_string();
            // For valid student data, ensure the default phone number is also valid or not checked here
            // Overwriting it like above is fine.
            assert!(student.validate().is_ok(), "Validation failed for a supposedly valid phone: {}", phone);
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

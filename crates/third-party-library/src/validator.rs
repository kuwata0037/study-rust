#[cfg(test)]
mod tests {
    use validator::{Validate, ValidationErrors};

    #[derive(Debug, Validate)]
    struct SignupData {
        #[validate(email)]
        mail: String,
        #[validate(phone)]
        phone: String,
        #[validate(url)]
        site: String,
        #[validate(length(min = 1))]
        first_name: String,
        #[validate(range(min = 10, max = 25))]
        age: u32,
    }

    #[test]
    fn test_validate_success() {
        let data = SignupData {
            mail: "luis.stephens@example.com".to_string(),
            phone: "+1 (413) 487-0806".to_string(),
            site: "https://example.com/test".to_string(),
            first_name: "Jennie".to_string(),
            age: 14,
        };
        data.validate().unwrap();
    }

    #[test]
    fn test_validate_success_for_japanese() {
        let data = SignupData {
            mail: "hiroshima_eri@example.com".to_string(),
            phone: "+81946267892".to_string(),
            site: "https://example.com/test".to_string(),
            first_name: "廣島".to_string(),
            age: 24,
        };
        data.validate().unwrap();
    }

    #[test]
    fn test_validate_failure() {
        let data = SignupData {
            mail: "hiroshima_eri/example.com".to_string(),
            phone: "+81 080-3681-6493".to_string(),
            site: "example.com/test".to_string(),
            first_name: "".to_string(),
            age: 30,
        };

        let result = data.validate();

        assert!(ValidationErrors::has_error(&result, "mail"));
        assert!(ValidationErrors::has_error(&result, "phone"));
        assert!(ValidationErrors::has_error(&result, "site"));
        assert!(ValidationErrors::has_error(&result, "first_name"));
        assert!(ValidationErrors::has_error(&result, "age"));
        match result {
            Err(errors) => {
                assert_eq!(5, errors.errors().len());
            }
            Ok(_) => unreachable!(),
        }
    }
}

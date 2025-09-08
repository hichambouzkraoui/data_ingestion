#[cfg(test)]
mod tests {
    use crate::infrastructure::parsers::avro_parser::parse_avro;
    use apache_avro::{Writer, Schema, types::Value};

    fn create_test_avro_data() -> Vec<u8> {
        let schema = Schema::parse_str(r#"
        {
            "type": "record",
            "name": "User",
            "fields": [
                {"name": "name", "type": "string"},
                {"name": "age", "type": "int"},
                {"name": "email", "type": "string"}
            ]
        }
        "#).unwrap();

        let mut writer = Writer::new(&schema, Vec::new());
        
        let user1 = vec![
            ("name".to_string(), Value::String("John Doe".to_string())),
            ("age".to_string(), Value::Int(25)),
            ("email".to_string(), Value::String("john@example.com".to_string()))
        ];
        
        let user2 = vec![
            ("name".to_string(), Value::String("Jane Smith".to_string())),
            ("age".to_string(), Value::Int(30)),
            ("email".to_string(), Value::String("jane@example.com".to_string()))
        ];

        writer.append(Value::Record(user1)).unwrap();
        writer.append(Value::Record(user2)).unwrap();
        writer.into_inner().unwrap()
    }

    #[test]
    fn test_parse_avro_success() {
        let avro_data = create_test_avro_data();
        let result = parse_avro(&avro_data).unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0]["name"], "John Doe");
        assert_eq!(result[0]["age"], 25);
        assert_eq!(result[1]["name"], "Jane Smith");
        assert_eq!(result[1]["age"], 30);
    }

    #[test]
    fn test_parse_invalid_avro() {
        let invalid_data = b"invalid avro data";
        let result = parse_avro(invalid_data);
        
        assert!(result.is_err());
    }
}
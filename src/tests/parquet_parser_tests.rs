#[cfg(test)]
mod tests {
    use crate::infrastructure::parsers::parquet_parser::parse_parquet;
    use arrow::array::{Int32Array, StringArray};
    use arrow::record_batch::RecordBatch;
    use arrow::datatypes::{DataType, Field, Schema};
    use parquet::arrow::ArrowWriter;
    use std::sync::Arc;

    fn create_test_parquet_data() -> Vec<u8> {
        let schema = Schema::new(vec![
            Field::new("name", DataType::Utf8, false),
            Field::new("age", DataType::Int32, false),
            Field::new("email", DataType::Utf8, false),
        ]);

        let name_array = StringArray::from(vec!["John Doe", "Jane Smith"]);
        let age_array = Int32Array::from(vec![25, 30]);
        let email_array = StringArray::from(vec!["john@example.com", "jane@example.com"]);

        let batch = RecordBatch::try_new(
            Arc::new(schema.clone()),
            vec![
                Arc::new(name_array),
                Arc::new(age_array),
                Arc::new(email_array),
            ],
        ).unwrap();

        let mut buffer = Vec::new();
        {
            let mut writer = ArrowWriter::try_new(&mut buffer, Arc::new(schema), None).unwrap();
            writer.write(&batch).unwrap();
            writer.close().unwrap();
        }
        buffer
    }

    #[test]
    fn test_parse_parquet_success() {
        let parquet_data = create_test_parquet_data();
        let result = parse_parquet(&parquet_data).unwrap();
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0]["name"], "John Doe");
        assert_eq!(result[0]["age"], 25);
        assert_eq!(result[1]["name"], "Jane Smith");
        assert_eq!(result[1]["age"], 30);
    }

    #[test]
    fn test_parse_invalid_parquet() {
        let invalid_data = b"invalid parquet data";
        let result = parse_parquet(invalid_data);
        
        assert!(result.is_err());
    }
}
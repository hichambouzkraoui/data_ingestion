use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use bytes::Bytes;
use std::collections::HashMap;
use tracing::{debug, info, error};
use crate::domain::error::IngestionError;

pub fn parse_parquet(bytes: &[u8]) -> Result<Vec<serde_json::Value>, IngestionError> {
    debug!("Creating Parquet reader");
    let bytes_data = Bytes::from(bytes.to_vec());
    
    let mut documents = Vec::new();
    let mut record_count = 0;

    let parquet_reader = ParquetRecordBatchReaderBuilder::try_new(bytes_data)
        .map_err(|e| {
            error!("Failed to create Parquet record batch reader: {}", e);
            IngestionError::Parse(e.to_string())
        })?
        .build()
        .map_err(|e| {
            error!("Failed to build Parquet record batch reader: {}", e);
            IngestionError::Parse(e.to_string())
        })?;

    for batch_result in parquet_reader {
        let batch = batch_result.map_err(|e| {
            error!("Failed to read Parquet batch: {}", e);
            IngestionError::Parse(e.to_string())
        })?;

        // Convert RecordBatch to JSON manually
        let schema = batch.schema();
        for row_idx in 0..batch.num_rows() {
            let mut row_map = HashMap::new();
            
            for (col_idx, field) in schema.fields().iter().enumerate() {
                let column = batch.column(col_idx);
                let value = match column.data_type() {
                    arrow::datatypes::DataType::Utf8 => {
                        let array = column.as_any().downcast_ref::<arrow::array::StringArray>().unwrap();
                        serde_json::Value::String(array.value(row_idx).to_string())
                    },
                    arrow::datatypes::DataType::Int32 => {
                        let array = column.as_any().downcast_ref::<arrow::array::Int32Array>().unwrap();
                        serde_json::Value::Number(serde_json::Number::from(array.value(row_idx)))
                    },
                    arrow::datatypes::DataType::Int64 => {
                        let array = column.as_any().downcast_ref::<arrow::array::Int64Array>().unwrap();
                        serde_json::Value::Number(serde_json::Number::from(array.value(row_idx)))
                    },
                    _ => serde_json::Value::String(format!("{:?}", column))
                };
                row_map.insert(field.name().clone(), value);
            }
            
            record_count += 1;
            documents.push(serde_json::Value::Object(serde_json::Map::from_iter(row_map)));
        }

        if record_count % 1000 == 0 {
            debug!("Processed {} Parquet records", record_count);
        }
    }

    info!("Parsed {} records from Parquet", record_count);
    Ok(documents)
}
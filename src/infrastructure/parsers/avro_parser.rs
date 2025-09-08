use apache_avro::{Reader, from_value};
use std::io::Cursor;
use tracing::{debug, info, error};
use crate::domain::error::IngestionError;

pub fn parse_avro(bytes: &[u8]) -> Result<Vec<serde_json::Value>, IngestionError> {
    debug!("Creating Avro reader");
    let cursor = Cursor::new(bytes);
    let reader = Reader::new(cursor)
        .map_err(|e| {
            error!("Failed to create Avro reader: {}", e);
            IngestionError::Parse(e.to_string())
        })?;

    let mut documents = Vec::new();
    let mut record_count = 0;

    for record in reader {
        let record = record.map_err(|e| {
            error!("Failed to read Avro record at position {}: {}", record_count, e);
            IngestionError::Parse(e.to_string())
        })?;

        record_count += 1;
        
        let json_value: serde_json::Value = from_value(&record)
            .map_err(|e| {
                error!("Failed to convert Avro record {} to JSON: {}", record_count, e);
                IngestionError::Parse(e.to_string())
            })?;

        documents.push(json_value);

        if record_count % 1000 == 0 {
            debug!("Processed {} Avro records", record_count);
        }
    }

    info!("Parsed {} records from Avro", record_count);
    Ok(documents)
}
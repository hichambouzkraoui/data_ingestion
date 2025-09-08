use crate::{
    domain::{error::IngestionError, ports::DataParser},
    infrastructure::parsers::{
        avro_parser::parse_avro,
        csv_parser::{parse_csv, parse_csv_with_config},
        excel_parser::parse_excel,
        json_parser::parse_json,
        parquet_parser::parse_parquet,
        txt_parser::parse_txt,
        xml_parser::parse_xml,
    },
};
use async_trait::async_trait;
use tracing::{debug, error, info};

pub struct ParserAdapter;

impl ParserAdapter {
    pub fn new() -> Self {
        debug!("Initializing parser adapter");
        Self
    }
}

#[async_trait]
impl DataParser for ParserAdapter {
    async fn parse(
        &self,
        file_bytes: &[u8],
        file_type: &str,
    ) -> Result<Vec<serde_json::Value>, IngestionError> {
        self.parse_with_config(file_bytes, file_type, None).await
    }

    async fn parse_with_config(
        &self,
        file_bytes: &[u8],
        file_type: &str,
        config: Option<&serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>, IngestionError> {
        info!(
            "Parsing file with type: {} ({} bytes)",
            file_type,
            file_bytes.len()
        );

        let result = match file_type {
            "avro" => {
                debug!("Parsing Avro file");
                parse_avro(file_bytes)
            }
            "csv" => {
                debug!("Parsing CSV file with config: {:?}", config);
                parse_csv_with_config(file_bytes, config)
            }
            "json" => {
                debug!("Parsing JSON file");
                parse_json(file_bytes)
            }
            "parquet" => {
                debug!("Parsing Parquet file");
                parse_parquet(file_bytes)
            }
            "txt" => {
                debug!("Parsing text file");
                parse_txt(file_bytes)
            }
            "xml" => {
                debug!("Parsing XML file");
                parse_xml(file_bytes)
            }
            "xls" | "xlsx" => {
                debug!("Parsing Excel file ({})", file_type);
                parse_excel(file_bytes)
            }

            _ => {
                error!("Unsupported file type: {}", file_type);
                Err(IngestionError::Parse(format!(
                    "Unsupported file type: {}",
                    file_type
                )))
            }
        };

        match &result {
            Ok(documents) => {
                info!(
                    "✅ Successfully parsed {} documents from {} file",
                    documents.len(),
                    file_type
                );
                debug!(
                    "Sample document: {}",
                    documents
                        .first()
                        .map(|d| serde_json::to_string_pretty(d)
                            .unwrap_or_else(|_| "<invalid json>".to_string()))
                        .unwrap_or_else(|| "<no documents>".to_string())
                );
            }
            Err(e) => {
                error!("❌ Failed to parse {} file: {}", file_type, e);
            }
        }

        result
    }
}

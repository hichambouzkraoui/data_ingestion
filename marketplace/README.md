# AWS Marketplace Package - Data Ingestion Application

## Overview
Automated file processing solution that ingests files from S3, processes them, and stores results in NoSQL databases.

## Supported File Types
- CSV (with/without headers)
- JSON
- XML
- TXT
- Excel (XLS/XLSX)

## Supported Databases
- MongoDB
- DocumentDB

## Architecture
- **ECS Fargate**: Serverless container execution
- **S3**: File storage and event triggers
- **SQS**: Message queuing for processing
- **Auto Scaling**: Scales based on queue depth

## Prerequisites
- VPC with private subnets
- MongoDB instance (if using MongoDB)
- S3 bucket permissions

## Deployment Steps

### 1. Prepare Container Image
```bash
# Build and push to your ECR
aws ecr create-repository --repository-name data-ingestion-marketplace
docker build -t data-ingestion .
docker tag data-ingestion:latest <account>.dkr.ecr.<region>.amazonaws.com/data-ingestion-marketplace:latest
docker push <account>.dkr.ecr.<region>.amazonaws.com/data-ingestion-marketplace:latest

# Store image URI in Parameter Store
aws ssm put-parameter \
  --name "/marketplace/data-ingestion/image-uri" \
  --value "<account>.dkr.ecr.<region>.amazonaws.com/data-ingestion-marketplace:latest" \
  --type "String"
```

### 2. Deploy CloudFormation Stack
```bash
aws cloudformation deploy \
  --template-file marketplace-template.yaml \
  --stack-name data-ingestion-marketplace \
  --parameter-overrides \
    DatabaseType=mongodb \
    MongoDBURI="mongodb://your-mongodb-uri" \
    VpcId=vpc-xxxxxxxx \
    SubnetIds=subnet-xxxxxxxx,subnet-yyyyyyyy \
  --capabilities CAPABILITY_IAM
```

## Configuration

### MongoDB Setup
1. Create MongoDB instance (Atlas, DocumentDB, or self-hosted)
2. Create database and collections:
   - `ingestion_logs` - Processing logs
   - `csv_data` - CSV file data
   - `json_data` - JSON file data
   - `xml_data` - XML file data
   - `text_logs` - Text file data

### File Processing Rules
The application uses pattern-based routing. Configure rules in your database:

```json
{
  "pattern": "^data/.*\\.csv$",
  "target_table": "csv_data",
  "parser_config": {
    "has_headers": true
  }
}
```

## Usage

1. **Upload files** to the S3 bucket
2. **S3 events** trigger SQS messages
3. **ECS tasks** auto-scale to process files
4. **Processed data** stored in configured database
5. **Logs** tracked in `ingestion_logs` collection

## Monitoring

- **CloudWatch Logs**: `/ecs/{stack-name}`
- **SQS Metrics**: Queue depth and processing rate
- **ECS Metrics**: Task count and resource usage

## Pricing

- **ECS Fargate**: Pay per task execution time
- **S3**: Standard storage and request pricing
- **SQS**: Message processing charges
- **CloudWatch**: Log storage and metrics

## Support

For technical support and customization:
- Documentation: [GitHub Repository]
- Issues: [GitHub Issues]
- Email: support@yourcompany.com
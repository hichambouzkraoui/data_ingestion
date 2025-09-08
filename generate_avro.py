#!/usr/bin/env python3
import sys
import json

try:
    import avro.schema
    import avro.io
    import avro.datafile
    
    def create_avro_file(filename):
        schema_dict = {
            "type": "record",
            "name": "User",
            "fields": [
                {"name": "name", "type": "string"},
                {"name": "age", "type": "int"},
                {"name": "email", "type": "string"}
            ]
        }
        
        schema = avro.schema.parse(json.dumps(schema_dict))
        
        with open(filename, 'wb') as f:
            writer = avro.io.DatumWriter(schema)
            file_writer = avro.datafile.DataFileWriter(f, writer, schema)
            
            # Write records
            file_writer.append({"name": "John Doe", "age": 25, "email": "john@example.com"})
            file_writer.append({"name": "Jane Smith", "age": 30, "email": "jane@example.com"})
            
            file_writer.close()
        
        print(f"Created Avro file: {filename}")
        return True
        
except ImportError:
    def create_avro_file(filename):
        print("avro-python3 not available, skipping Avro test")
        # Create a dummy file that will fail parsing gracefully
        with open(filename, 'w') as f:
            f.write("dummy avro file")
        return False

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 generate_avro.py <filename>")
        sys.exit(1)
    
    create_avro_file(sys.argv[1])
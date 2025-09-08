#!/usr/bin/env python3
import sys

try:
    import pyarrow as pa
    import pyarrow.parquet as pq
    
    def create_parquet_file(filename):
        # Create sample data
        data = {
            'name': ['John Doe', 'Jane Smith', 'Bob Johnson'],
            'age': [25, 30, 35],
            'email': ['john@example.com', 'jane@example.com', 'bob@example.com']
        }
        
        # Create Arrow table
        table = pa.table(data)
        
        # Write to Parquet file
        pq.write_table(table, filename)
        
        print(f"Created Parquet file: {filename}")
        return True
        
except ImportError:
    def create_parquet_file(filename):
        print("pyarrow not available, creating dummy file")
        with open(filename, 'w') as f:
            f.write("dummy parquet file")
        return False

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 generate_parquet.py <filename>")
        sys.exit(1)
    
    create_parquet_file(sys.argv[1])
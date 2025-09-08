#!/usr/bin/env python3
"""Generate real Excel files for testing"""

try:
    from openpyxl import Workbook
    from openpyxl.utils.dataframe import dataframe_to_rows
except ImportError:
    print("openpyxl not found. Installing...")
    import subprocess
    import sys
    subprocess.check_call([sys.executable, "-m", "pip", "install", "--break-system-packages", "openpyxl"])
    from openpyxl import Workbook

import sys
import os

def create_excel_file(filename):
    """Create a real Excel file with test data"""
    wb = Workbook()
    ws = wb.active
    ws.title = "TestData"
    
    # Add headers
    headers = ["name", "age", "department", "salary"]
    ws.append(headers)
    
    # Add test data
    test_data = [
        ["Alice", 28, "HR", 65000],
        ["Charlie", 32, "Finance", 75000],
        ["Diana", 29, "Engineering", 85000],
        ["Bob", 35, "Marketing", 70000]
    ]
    
    for row in test_data:
        ws.append(row)
    
    wb.save(filename)
    print(f"Created Excel file: {filename}")

if __name__ == "__main__":
    filename = sys.argv[1] if len(sys.argv) > 1 else "data/test.xlsx"
    os.makedirs(os.path.dirname(filename), exist_ok=True)
    create_excel_file(filename)
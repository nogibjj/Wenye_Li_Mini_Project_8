import pandas as pd
import psutil
import time
import os
import requests
import numpy as np


def get_memory_usage():
    """Get current process memory usage in MB"""
    process = psutil.Process()
    return process.memory_info().rss / 1024 / 1024


def extract(url, file_path):
    """Extract a url to a file path"""
    start_time = time.perf_counter()
    start_memory = get_memory_usage()

    directory = os.path.dirname(file_path)
    if not os.path.exists(directory):
        os.makedirs(directory)

    with requests.get(url) as r:
        with open(file_path, "wb") as f:
            f.write(r.content)

    end_time = time.perf_counter()
    end_memory = get_memory_usage()

    return {
        "operation": "Extract",
        "time": end_time - start_time,
        "memory": end_memory - start_memory,
    }


def calculate_means(file_path):
    """Calculate means for each numeric column"""
    start_time = time.perf_counter()
    start_memory = get_memory_usage()

    # Read CSV
    df = pd.read_csv(file_path)

    # Calculate means for each numeric column
    means = {}
    for column in df.columns:
        try:
            # Skip non-numeric columns and handle missing values
            if column in ["age"]:  # skip non-numeric columns
                continue
            # Replace "-" with np.nan instead of pd.NA
            means[column] = (
                pd.to_numeric(df[column].replace("-", np.nan), errors="coerce")
                .mean()
            )
        except ValueError as e:
            print(f"Error processing column {column}: {e}")
            continue

    end_time = time.perf_counter()
    end_memory = get_memory_usage()

    perf_metrics = {
        "operation": "Calculate Means",
        "time": end_time - start_time,
        "memory": end_memory - start_memory,
    }

    return means, perf_metrics


def main():
    # Initialize performance metrics list
    performance_metrics = []

    # Extract data
    url = (
        "https://raw.githubusercontent.com/fivethirtyeight/data/master/"
        "drug-use-by-age/drug-use-by-age.csv"
    )
    file_path = "data/drug-use-by-age.csv"

    extract_metrics = extract(url, file_path)
    performance_metrics.append(extract_metrics)

    # Calculate means
    means, calc_metrics = calculate_means(file_path)
    performance_metrics.append(calc_metrics)

    # Create performance report
    report = "# Python Performance Report\n\n"
    report += "## Column Means\n\n"
    for column, mean in means.items():
        report += f"- {column}: {mean:.2f}\n"

    report += "\n## Performance Metrics\n\n"
    report += "| Operation | Time (s) | Memory (MB) |\n"
    report += "|-----------|----------|-------------|\n"

    # Format performance metrics table with line breaks for readability
    for metric in performance_metrics:
        report += (
            f"| {metric['operation']:<9} "
            f"| {metric['time']:>8.6f} "
            f"| {metric['memory']:>11.2f} "
            f"|\n"
        )

    # Write report to file
    with open("Python_Performance.md", "w") as f:
        f.write(report)

    return performance_metrics


# if __name__ == "__main__":
#     main()
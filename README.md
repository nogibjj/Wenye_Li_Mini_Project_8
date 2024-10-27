[![Rust CI/CD Pipeline](https://github.com/nogibjj/Wenye_Li_Mini_Project_8/actions/workflows/rust_cicd.yml/badge.svg)](https://github.com/nogibjj/Wenye_Li_Mini_Project_8/actions/workflows/rust_cicd.yml)
[![Python CI/CD Pipeline](https://github.com/nogibjj/Wenye_Li_Mini_Project_8/actions/workflows/python_cicd.yml/badge.svg)](https://github.com/nogibjj/Wenye_Li_Mini_Project_8/actions/workflows/python_cicd.yml)

## Wenye Li Mini Project 8

This project implements a data processing tool in both Python and Rust to analyze drug use statistics. The tool downloads data from FiveThirtyEight's dataset, processes it, and calculates mean values for various drug use metrics across age groups.

## Requirements

- Take an existing Python script for data processing
- Rewrite it in Rust
- Highlight improvements in speed and resource usage

## Deliverables

- Rust and Python scripts
- Performance comparison report (PDF or markdown)

## Features

- Data extraction from URL
- Mean calculation for numeric columns
- Performance metrics tracking (time and memory usage)
- Markdown report generation

## Setup

### Python Version

- Install dependencies: `make install`
- Formart codes: `make format`
- Lint codes: `make lint`
- Test code: `make test`

### Rust Version

- Build the Rust project: `make rust_build`
- Run the Rust project: `make rust_run`
- Formart Rust codes: `make rust_format`
- - Lint Rust codes: `make rust_lint`
- Run tests for the Rust project: `make rust_test`

### Execution Time

| Operation       | Python (s) | Rust (s) | Improvement |
| --------------- | ---------- | -------- | ----------- |
| Extract         | 0.191566   | 0.166017 | 13.3%       |
| Calculate Means | 0.008319   | 0.003032 | 63.6%       |
| Total           | 0.201893   | 0.169109 | 16.2%       |

- Python Total Time: 0.201893 seconds
- Rust Total Time: 0.169109 seconds
- Improvement: 16.2% faster in Rust
- Most significant in mean calculation (63.6% faster)

### Memory Usage

| Operation       | Python (MB) | Rust (MB) |
| --------------- | ----------- | --------- |
| Extract         | 2.84        | 6.19      |
| Calculate Means | 0.72        | 0.69      |
| Total           | 3.56        | 6.88      |

- Rust shows higher memory usage in this implementation
- Possible reasons:
  - Different memory allocation strategies
  - CSV parsing implementation differences

## Key Findings:

- Rust excels in computational tasks
- Network operations show smaller performance gap

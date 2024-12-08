Check CI/CD Status:

# Mini-project #8
#### Repo Title: Rewrite a Python Script in Rust
#### Author: Seijung Kim (sk591)

## Overview
This project aims to compare the performances of data processing scripts written in Python and Rust. The Python and Rust functions process the Titanic data, calculate the average age of passengers who survived, and generate a performance report. The goal is to demonstrate that rewriting a Python data processing script in Rust has improvements in execution speed and memory usage. This comparison highlights the efficiency of Rust over Python in handling data-intensive tasks, providing insights into potential benefits when using Rust for similar workloads. Check the Performance Comparison Report for details about the performance metrics.

## Requirements/Deliverables
* Take an existing Python script for data processing and rewrite it in Rust
* Highlight improvements in speed and resource usage
* Performance comparison report (PDF or markdown)

## Summary of Performance Comparison  
| Metric            | Python                | Rust                  |
|-------------------|-----------------------|-----------------------|
| Execution Time    | 0.66 ms               | 0.10 ms               |
| Memory Usage      | 41 KB                 | 16384 KB              |
| Average Age       | 25.27                 | 0.00                  |

### Project Outcome:
The comparison clearly illustrates that the Rust implementation is significantly faster than Python, with an execution time over 6 times faster than the Python version. While Python's memory usage was relatively low, the Rust implementation processed the data more efficiently. However, it also utilized more memory in the process, which may be due to the way memory is allocated and managed in Rust versus Python's garbage collection.

This result shows that while Rust can provide performance benefits in execution speed, there is a trade-off in memory consumption. These factors need to be considered when deciding which language to use for specific types of data processing tasks.
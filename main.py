import csv
import time
from memory_profiler import memory_usage


def filter_and_average_age(file_path, survived_threshold):
    count = 0
    age_sum = 0
    try:
        with open(file_path, newline="", encoding="utf-8") as csvfile:
            reader = csv.DictReader(csvfile)
            for row in reader:
                survived = int(row["Survived"])
                age = row["Age"]

                if survived == survived_threshold:
                    if age:  # Skip rows with missing age
                        try:
                            age = float(age)  # Try converting age to float
                            age_sum += age
                            count += 1
                        except ValueError:
                            print(f"Skipping row due to invalid age: {row}")
                    else:
                        # Handle missing age gracefully
                        print(f"Skipping row due to missing age: {row}")
                        # Optionally, set a default value for missing age (e.g., 30)
                        # age_sum += 30  # Uncomment this to impute a default age
                        # count += 1  # Uncomment this if you want to count the row with default age

        # Debug: check the age sum and count before calculating the average
        print(f"Age sum: {age_sum}, Count: {count}")

    except FileNotFoundError as e:
        print(f"Error: {e}")
        return 0.0
    except ValueError as e:
        print(f"Value error: {e}")
        return 0.0

    return float(age_sum / count) if count > 0 else 0.0  # Return as float


def generate_performance_report(exec_time_s, memory_usage_kb, avg_age):
    exec_time_ms = exec_time_s * 1000
    with open("python_performance_report.md", "w", encoding="utf-8") as file:
        file.write("# Python Performance Report\n")
        file.write("| Metric            | Python                |\n")
        file.write("|-------------------|-----------------------|\n")
        file.write(f"| Execution Time    | {exec_time_ms:.2f} ms               |\n")
        file.write(f"| Memory Usage      | {int(memory_usage_kb)} KB             |\n")
        file.write(f"| Average Age       | {avg_age:.2f}               |\n")


def main():
    # Define variables in main
    file_path = "data/titanic.csv"  # Path to Titanic dataset
    survived_threshold = 1  # Consider only survivors

    # Track memory usage with correct argument
    peak_memory_usage = max(
        memory_usage((filter_and_average_age, (file_path, survived_threshold)))
    )
    peak_memory_kb = peak_memory_usage * 1024

    # Start measuring execution time
    start_time = time.time()
    calculated_avg_age = filter_and_average_age(file_path, survived_threshold)
    end_time = time.time()
    execution_time = end_time - start_time

    # Generate performance report
    generate_performance_report(execution_time, peak_memory_kb, calculated_avg_age)


if __name__ == "__main__":
    main()

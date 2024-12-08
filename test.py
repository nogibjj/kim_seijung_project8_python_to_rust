# test.py
import pytest
from main import filter_and_average_age

# Test to verify average age calculation
def test_filter_and_average_age():
    file_path = "data/titanic.csv"
    survived_threshold = 1  # testing survivors
    avg_age = filter_and_average_age(file_path, survived_threshold)

    # Assert that the result is a float and greater than 0
    assert isinstance(avg_age, float), f"Expected a float, got {type(avg_age)}"
    assert avg_age > 0, f"Expected average age > 0, but got {avg_age}"


if __name__ == "__main__":
    pytest.main()

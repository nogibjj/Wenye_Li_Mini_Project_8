"""
Test goes here

"""

import subprocess


def test_extract():
    """tests extract()"""
    result = subprocess.run(
        ["python", "main.py", "extract"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0
    assert "Extracting data..." in result.stdout


def test_transform_load():
    """tests transfrom_load"""
    result = subprocess.run(
        ["python", "main.py", "transform_load"],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0
    assert "Transforming data..." in result.stdout


def test_run_query():
    """tests general_query"""
    result = subprocess.run(
        [
            "python",
            "main.py",
            "run_query",
            """
            WITH AgeStats AS (
            SELECT age,
            AVG(alcohol_use) AS avg_alcohol_use,
            AVG(marijuana_use) AS avg_marijuana_use
            FROM DrugUseDB
            GROUP BY age
            )
            SELECT d.age, d.n, d.alcohol_use, a.avg_alcohol_use, 
            d.marijuana_use, a.avg_marijuana_use
            FROM DrugUseDB d
            JOIN AgeStats a
            ON d.age = a.age
            ORDER BY d.age ASC, d.n DESC
            """,
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    assert result.returncode == 0


if __name__ == "__main__":
    test_extract()
    test_transform_load()
    test_run_query()
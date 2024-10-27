import subprocess
import os

def test_performance_report():
    """Tests the main script's execution and output."""
    # Run the main script
    result = subprocess.run(
        ["python", "main.py"],
        capture_output=True,
        text=True,
        check=True,
    )
    
    # Check if the script executed successfully
    assert result.returncode == 0, "Script failed to execute"
    
    # Check if the performance report file was created
    assert os.path.exists("Python_Performance.md"), "Performance report file not created"
    
    # Read the performance report and check its contents
    with open("Python_Performance.md", "r") as f:
        report_content = f.read()
        
    # Check if the report contains essential sections
    assert "Python Performance Report" in report_content
    assert "Column Means" in report_content
    assert "Performance Metrics" in report_content
    assert "Operation" in report_content
    assert "Time (s)" in report_content

if __name__ == "__main__":
    test_performance_report()
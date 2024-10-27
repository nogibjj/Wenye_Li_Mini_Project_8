#[cfg(test)]
mod tests {
    use drug_use_analysis::{extract, calculate_means, validate_file};
    use std::fs;

    const TEST_URL: &str = "https://raw.githubusercontent.com/fivethirtyeight/data/master/drug-use-by-age/drug-use-by-age.csv";
    const TEST_FILE: &str = "data/test_drug-use-by-age.csv";

    #[test]
    fn test_extract() {
        // Test file extraction
        let result = extract(TEST_URL, TEST_FILE);
        assert!(result.is_ok(), "Failed to extract file");
        
        // Verify file exists and has content
        let file_valid = validate_file(TEST_FILE).unwrap();
        assert!(file_valid, "Extracted file is empty or doesn't exist");
        
        // Clean up
        let _ = fs::remove_file(TEST_FILE);
    }

    #[test]
    fn test_calculate_means() {
        // First extract the test file
        let _ = extract(TEST_URL, TEST_FILE);
        
        // Calculate means
        let means = calculate_means(TEST_FILE);
        assert!(means.is_ok(), "Failed to calculate means");
        
        let means = means.unwrap();
        assert!(!means.is_empty(), "Means calculation returned empty results");
        
        // Check some expected columns
        assert!(means.contains_key("alcohol_use"), "Missing alcohol_use column");
        assert!(means.contains_key("marijuana_use"), "Missing marijuana_use column");

        let _ = fs::remove_file(TEST_FILE);
    }
}
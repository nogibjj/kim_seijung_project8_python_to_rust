#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    // Test that the final message printed by the main.rs is correct
    #[test]
    fn test_performance_report_message() {
        let mut cmd = Command::cargo_bin("sqlite").unwrap();

        // Run the binary (this will execute main.rs)
        let output = cmd
            .assert()
            .success()
            .stdout(predicates::str::contains(
                "Performance report generated at rust_performance_report.md",
            ))
            .stderr("");

        // Ensure the final message contains the expected text
        assert!(String::from_utf8_lossy(&output.get_output().stdout)
            .contains("Performance report generated at rust_performance_report.md"));
    }
}

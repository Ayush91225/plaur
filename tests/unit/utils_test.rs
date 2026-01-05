use plaur_common::utils::{calculate_percentage_change, format_currency, sanitize_string};

#[test]
fn test_calculate_percentage_change() {
    assert_eq!(calculate_percentage_change(100.0, 110.0), 10.0);
    assert_eq!(calculate_percentage_change(100.0, 90.0), -10.0);
    assert_eq!(calculate_percentage_change(0.0, 100.0), 0.0);
}

#[test]
fn test_format_currency() {
    assert_eq!(format_currency(123.45, "USD"), "$123.45");
    assert_eq!(format_currency(123.45, "EUR"), "€123.45");
    assert_eq!(format_currency(123.45, "GBP"), "£123.45");
    assert_eq!(format_currency(123.45, "INR"), "₹123.45");
}

#[test]
fn test_sanitize_string() {
    assert_eq!(sanitize_string("Hello World!"), "Hello World");
    assert_eq!(sanitize_string("Test-123_abc"), "Test-123_abc");
    assert_eq!(sanitize_string("Special@#$%"), "Special");
}
use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_path_home() {
        let path = expand_path("~/tests");
        let path_str = path.to_string_lossy();
        // Verify that the tilde was expanded correctly to an absolute path
        assert!(!path_str.starts_with('~'));
        assert!(path.is_absolute());
    }

    #[test]
    fn test_security_path_traversal_and_null_byte() {
        // Cybersec: Injection test for nulled bytes and path traversal
        // Rust is memory safe, but we validate that the function returns the path
        // without panicking or corrupting it when receiving malicious input.
        let malicius_input = "../../../etc/passwd\0archivo_falso.png";
        let path = expand_path(malicius_input);
        let path_str = path.to_string_lossy();
        
        assert!(path_str.contains("../../../etc/passwd"));
        assert!(path_str.contains('\0'));
    }

    #[test]
fn test_get_default_ascii_not_empty() {
    let ascii = get_default_ascii();
    assert!(!ascii.is_empty());
    assert!(ascii.contains("linux"));
}

#[test]
fn test_expand_path_normal() {
    let path = expand_path("/usr/bin/custom_logo.png");
    assert_eq!(path.to_string_lossy(), "/usr/bin/custom_logo.png");
}
}
/// Unit tests for loading networks

use crate::load::*;

#[test]
fn should_return_error_on_missing_file() {
    let res = load_network("non-existing-file.txt".to_string());
    assert!(res.is_err(), "Should return Error for non existing file");
}

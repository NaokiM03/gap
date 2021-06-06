use std::fs;

use gap::Config;

#[test]
fn ron() {
    let args = "--f foo_value --bar bar_value -ab --ccc param_value".to_string();
    let ron = fs::read_to_string("./tests/sample.ron");
    if let Ok(ron) = ron {
        let result = Config::ron_to_result(ron, args);

        if let Some(result) = result {
            assert_eq!(result.value_of("foo"), Some("foo_value"));
            assert_eq!(result.value_of("bar"), Some("bar_value"));
            assert_eq!(result.value_of("param"), Some("param_value"));
            assert!(result.is_present("aaa"));
            assert!(result.is_present("bbb"));
            assert!(result.is_present("ccc"));
        }
    }
}

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

#[macro_export]
macro_rules! include_http {
    ($package: tt) => {
        include!(concat!(env!("OUT_DIR"), concat!("/", $package, ".rs")));
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_variable_value() {
        let params = HashMap::new();
        let result = get_variable_value(&params, "randomInt");
        println!("{}", result);
    }
}

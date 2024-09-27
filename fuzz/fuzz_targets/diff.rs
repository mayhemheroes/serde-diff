#![no_main]
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};
use serde_diff::{Apply, Diff, SerdeDiff};

#[derive(SerdeDiff, Serialize, Deserialize, PartialEq, Debug)]
struct TestStruct {
    a: u32,
    b: f64,
}

fuzz_target!(|value: (u32, f64, u32, f64, u32, f64)| {
    let old = TestStruct{ a: value.0, b: value.1 };
    let new = TestStruct{ a: value.2, b: value.3 };
    let mut target = TestStruct{ a: value.4, b: value.5 };
    let json_data = serde_json::to_string(&Diff::serializable(&old, &new)).unwrap();
    let mut deserializer = serde_json::Deserializer::from_str(&json_data);
    let _ = Apply::apply(&mut deserializer, &mut target);
});

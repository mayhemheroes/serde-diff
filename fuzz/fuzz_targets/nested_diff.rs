#![no_main]
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};
use serde_diff::{Apply, Diff, SerdeDiff};

#[derive(SerdeDiff, Serialize, Deserialize, PartialEq, Debug)]
struct TestInnerStruct {
    d: f64,
}

#[derive(SerdeDiff, Serialize, Deserialize, PartialEq, Debug)]
struct TestStruct {
    a: u32,
    b: f64,
    c: TestInnerStruct,
}

fuzz_target!(|value: (u32, f64, f64, u32, f64, f64, u32, f64, f64)| {
    let old = TestStruct{ a: value.0, b: value.1, c: TestInnerStruct{ d: value.2 } };
    let new = TestStruct{ a: value.3, b: value.4, c: TestInnerStruct{ d: value.5 } };
    let mut target = TestStruct{ a: value.6, b: value.7, c: TestInnerStruct{ d: value.8 } };
    let json_data = serde_json::to_string(&Diff::serializable(&old, &new)).unwrap();
    let mut deserializer = serde_json::Deserializer::from_str(&json_data);
    let _ = Apply::apply(&mut deserializer, &mut target);
});

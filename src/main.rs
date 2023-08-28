use serde_json::Value;

fn main() {
    let data = r#"
    {
        "metadata": {
            "annotations": {
                "thing": "stuff"
            },
            "name": "thing"
        },
        "spec":{"foo": "bar"}
    }"#;
    let v: Value = serde_json::from_str(data).unwrap();

    println!("annotations: {}", v["metadata"]["annotations"]);
}

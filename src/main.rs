use serde_json::{Value, json};

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
    let mut v: Value = serde_json::from_str(data).unwrap();
    add_annotation(&mut v);
    println!("annotations: {}", v["metadata"]["annotations"]);

    let data = r#"
    {
        "metadata": {
            "name": "thing"
        },
        "spec":{"foo": "bar"}
    }"#;
    let mut v: Value = serde_json::from_str(data).unwrap();
    add_annotation(&mut v);
    println!("annotations: {}", v["metadata"]["annotations"]);

    let data = r#"
    {
        "spec":{"foo": "bar"}
    }"#;
    let mut v: Value = serde_json::from_str(data).unwrap();
    add_annotation(&mut v);
    println!("annotations: {}", v["metadata"]["annotations"]);
}

fn add_annotation(
    resource: &mut Value
) {
    if let Some(annotations) = resource.pointer_mut("/metadata/annotations") {
        // annotations exist, add new key
        if let Some(obj) = annotations.as_object_mut() {
            obj.insert(String::from("recert-edited"), json!("/some/json/path/here"));
        }
    } else {
        // annotations don't exist add new map at annotations key in metadata
        let mut a = serde_json::Map::new();
        a.insert(String::from("recert-edited"), json!("/some/json/path/here"));
        if let Some(metadata) = resource.pointer_mut("/metadata") {
            if let Some(obj) = metadata.as_object_mut() {
                obj.insert(String::from("annotations"), Value::Object(a));
            }
        } else {
            let mut metadata = serde_json::Map::new();
            metadata.insert(String::from("annotations"), Value::Object(a));
            if let Some(obj) = resource.as_object_mut() {
                obj.insert(String::from("metadata"), Value::Object(metadata));
            }
        }
    }
}

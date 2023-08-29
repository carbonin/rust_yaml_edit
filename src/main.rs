use anyhow::{Context, Result};
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
    add_annotation(&mut v).unwrap_or_else(|err| {
        println!("failed to add annotation: {}", err);
    });
    println!("annotations: {}", v["metadata"]["annotations"]);

    let data = r#"
    {
        "metadata": {
            "annotations": "baz",
            "name": "thing"
        },
        "spec":{"foo": "bar"}
    }"#;
    let mut v: Value = serde_json::from_str(data).unwrap();
    add_annotation(&mut v).unwrap_or_else(|err| {
        println!("failed to add annotation: {}", err);
    });
    println!("annotations: {}", v["metadata"]["annotations"]);

    let data = r#"
    {
        "metadata": {
            "name": "thing"
        },
        "spec":{"foo": "bar"}
    }"#;
    let mut v: Value = serde_json::from_str(data).unwrap();
    add_annotation(&mut v).unwrap_or_else(|err| {
        println!("failed to add annotation: {}", err);
    });
    println!("annotations: {}", v["metadata"]["annotations"]);

    let data = r#"
    {
        "metadata": "stuff",
        "spec":{"foo": "bar"}
    }"#;
    let mut v: Value = serde_json::from_str(data).unwrap();
    add_annotation(&mut v).unwrap_or_else(|err| {
        println!("failed to add annotation: {}", err);
    });
    println!("annotations: {}", v["metadata"]["annotations"]);

    let data = r#"
    {
        "spec":{"foo": "bar"}
    }"#;
    let mut v: Value = serde_json::from_str(data).unwrap();
    add_annotation(&mut v).unwrap_or_else(|err| {
        println!("failed to add annotation: {}", err);
    });
    println!("annotations: {}", v["metadata"]["annotations"]);
}

fn add_annotation(resource: &mut Value) -> Result<()> {
    if resource.pointer_mut("/metadata/annotations").is_none() {
        resource
            .pointer_mut("/metadata")
            .context("metadata must exist")?
            .as_object_mut()
            .context("metadata must be an object")?
            .insert(
                String::from("annotations"),
                Value::Object(serde_json::Map::new()),
            );
    }

    resource
        .pointer_mut("/metadata/annotations")
        .context("annotations must exist")?
        .as_object_mut()
        .context("annotations must be an object")?
        .insert(String::from("recert-edited"), json!("/some/json/path/here"));

    Ok(())
}

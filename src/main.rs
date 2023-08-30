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

    // recert already added annotation
    let data = r#"
    {
        "metadata": {
            "annotations": {
                "thing": "stuff",
                "recert-edited": "{\"/spec/foo\": [1]}"
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

    // get the annotation content into a Map
    let mut m = serde_json::Map::new();
    if let Some(annotation_data) = resource.pointer_mut("/metadata/annotations/recert-edited") {
        m = serde_json::from_str(annotation_data.as_str().context("expected annotation data to be a string")?)?
                .as_object_mut().context("expected annotation value to be an object")?
    };

    // find the key for this location (/spec/foo)
    let index = Value::Number(serde_json::Number::from(5));
    match m.get("/spec/foo") {
        Some(locations) => {
            locations
                .as_array()
                .context("locations must be an array")?
                .push(index);
        },
        None => {
            m.insert(String::from("/spec/foo"), Value::Array(vec![index]));
        },
    }

    resource
        .pointer_mut("/metadata/annotations")
        .context("annotations must exist")?
        .as_object_mut()
        .context("annotations must be an object")?
        .insert(String::from("recert-edited"), Value::String(String::from(Value::Object(m).as_str().context("must translate object into string")?)));

    Ok(())
}

use anyhow::{Context, Result};
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
    // find the key for this location (/spec/foo) - passed in in real code
    let index = Value::Number(serde_json::Number::from(5));

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

    // get the string in the annotation key
    let s = match resource.pointer_mut("/metadata/annotations/recert-edited") {
        Some(annotation_data) => {
            annotation_data.as_str().context("expected annotation data to be a string")?
        },
        None => "{}"
    };

    // parse the json string into a value
    let mut annotation_value: Value = serde_json::from_str(s).context("annotation data must be valid json")?;

    match annotation_value.get_mut("/spec/foo") {
        Some(locations) => {
            locations
                .as_array_mut()
                .context("locations must be an array")?
                .push(index);
        },
        None => {
            annotation_value
                .as_object_mut()
                .context("annotation value must be an object")?
                .insert(String::from("/spec/foo"), Value::Array(vec![index]));
        },
    }

    let new_annotation_value = Value::String(serde_json::to_string(&annotation_value).context("failed to serialize new annotation value")?);
    resource
        .pointer_mut("/metadata/annotations")
        .context("annotations must exist")?
        .as_object_mut()
        .context("annotations must be an object")?
        .insert(String::from("recert-edited"), new_annotation_value);

    Ok(())
}

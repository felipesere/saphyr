use saphyr::{LoadableYamlNode, Scalar, Yaml, YamlEmitter};

fn roundtrip(original: &Yaml) {
    let mut emitted = String::new();
    YamlEmitter::new(&mut emitted).dump(original).unwrap();

    let documents = Yaml::load_from_str(&emitted).unwrap();
    println!("emitted {emitted}");

    assert_eq!(documents.len(), 1);
    assert_eq!(documents[0], *original);
}

#[allow(clippy::similar_names)]
fn roundtrip_multiline(original: &Yaml) {
    let mut emitted = String::new();
    let mut emitter = YamlEmitter::new(&mut emitted);
    emitter.multiline_strings(true);
    emitter.dump(original).unwrap();

    let documents = Yaml::load_from_str(&emitted).unwrap();
    println!("emitted {emitted}");

    assert_eq!(documents.len(), 1);
    assert_eq!(documents[0], *original);
}

fn double_roundtrip(original: &str) {
    let parsed = Yaml::load_from_str(original).unwrap();

    let mut serialized = String::new();
    YamlEmitter::new(&mut serialized).dump(&parsed[0]).unwrap();

    let reparsed = Yaml::load_from_str(&serialized).unwrap();

    assert_eq!(parsed, reparsed);
}

#[test]
fn test_escape_character() {
    let y = Yaml::Value(Scalar::String("\x1b".into()));
    roundtrip(&y);
}

#[test]
fn test_colon_in_string() {
    let y = Yaml::Value(Scalar::String("x: %".into()));
    roundtrip(&y);
}

#[test]
fn test_numberlike_strings() {
    let docs = [
        r#"x: "1234""#,
        r#"x: "01234""#,
        r#""1234""#,
        r#""01234""#,
        r#"" 01234""#,
        r#""0x1234""#,
        r#"" 0x1234""#,
    ];

    for doc in &docs {
        roundtrip(&Yaml::Value(Scalar::String((*doc).into())));
        double_roundtrip(doc);
    }
}

/// Example from <https://github.com/chyh1990/yaml-rust/issues/133>
#[test]
fn test_issue133() {
    let doc = Yaml::load_from_str("\"0x123\"").unwrap().pop().unwrap();
    assert_eq!(doc, Yaml::Value(Scalar::String("0x123".into())));

    let mut out_str = String::new();
    YamlEmitter::new(&mut out_str).dump(&doc).unwrap();
    let doc2 = Yaml::load_from_str(&out_str).unwrap().pop().unwrap();
    assert_eq!(doc, doc2); // This failed because the type has changed to a number now
}

#[test]
fn test_newline() {
    let y = Yaml::Sequence(vec![Yaml::Value(Scalar::String("\n".into()))]);
    roundtrip(&y);
}

#[test]
fn test_crlf() {
    let y = Yaml::Sequence(vec![Yaml::Value(Scalar::String("\r\n".into()))]);
    roundtrip(&y);
}

#[test]
fn test_multiline_noline() {
    let y = Yaml::Sequence(vec![Yaml::Value(Scalar::String("a".into()))]);
    roundtrip_multiline(&y);
}

#[test]
fn test_multiline_inner_newline() {
    let y = Yaml::Sequence(vec![Yaml::Value(Scalar::String("a\nb".into()))]);
    roundtrip_multiline(&y);
}

#[test]
fn test_multiline_trailing_newline() {
    let y = Yaml::Sequence(vec![Yaml::Value(Scalar::String("a\n".into()))]);
    roundtrip_multiline(&y);
}

#[test]
fn test_multiline_leading_newline() {
    let y = Yaml::Sequence(vec![Yaml::Value(Scalar::String("\na".into()))]);
    roundtrip_multiline(&y);
}

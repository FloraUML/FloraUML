use flora::generate;

// TODO actual integration testing of the executable

fn run(input: &str) -> String {
    let mut bytes = Vec::new();
    assert!(generate(&input, &mut bytes).is_ok());
    String::from_utf8(bytes).expect("Generated DOT is invalid UTF-8")
}

#[test]
fn zero() {
    assert_eq!(
        run(""),
        "graph g {
}
"
    );
}

#[test]
fn one() {
    assert_eq!(
        run("class A;"),
        r#"graph g {
    node_A[label="A"];
}
"#
    );
}

#[test]
fn three() {
    assert_eq!(
        run("
class A;
class B;
class C;
"),
        r#"graph g {
    node_A[label="A"];
    node_B[label="B"];
    node_C[label="C"];
    node_A -- node_B[label=""];
    node_A -- node_C[label=""];
    node_B -- node_C[label=""];
}
"#
    );
}

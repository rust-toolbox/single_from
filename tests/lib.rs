#[macro_use]
extern crate single_from;

#[test]
fn single_struct() {
    #[derive(SingleFrom)]
    struct SingleStruct {
        s: String
    }

    assert!(match SingleStruct::from("qwerty".to_owned()) {
        SingleStruct{ s } => s == "qwerty"
    });
    assert!(!(match SingleStruct::from("qwerty".to_owned()) {
        SingleStruct{ s } => s == "data"
    }));
}

#[test]
fn single_tuple() {
    #[derive(SingleFrom)]
    struct SingleTuple(String);

    assert!(match SingleTuple::from("qwerty".to_owned()) {
        SingleTuple(s) => s == "qwerty"
    });
    assert!(!(match SingleTuple::from("qwerty".to_owned()) {
        SingleTuple(s) => s == "data"
    }));
}

#[test]
fn enum_with_tuples() {
    #[derive(SingleFrom)]
    enum EnumWithTuples {
        Str(String),
        Int(i32)
    }

    assert!(match EnumWithTuples::from(12) {
        EnumWithTuples::Int(x) => x == 12,
        _ => false
    });
    assert!(match EnumWithTuples::from("qwerty".to_owned()) {
        EnumWithTuples::Str(s) => s == "qwerty",
        _ => false
    });
}

#[test]
fn enum_with_structs() {
    #[derive(SingleFrom)]
    enum EnumWithStructs {
        Str { s: String },
        Int { x: i32 }
    }

    assert!(match EnumWithStructs::from(12) {
        EnumWithStructs::Int { x } => x == 12,
        _ => false
    });
    assert!(match EnumWithStructs::from("qwerty".to_owned()) {
        EnumWithStructs::Str { s } => s == "qwerty",
        _ => false
    });
}

#[test]
#[allow(dead_code)]
fn enum_with_mixed() {
    #[derive(SingleFrom)]
    enum EnumWithMixed {
        Str { s: String },
        Int(i32),
        SomeUnit,
        #[not_generate_from]
        Pair(i32, i32)
    }

    assert!(match EnumWithMixed::from(12) {
        EnumWithMixed::Int(x) => x == 12,
        _ => false
    });
    assert!(match EnumWithMixed::from("qwerty".to_owned()) {
        EnumWithMixed::Str { s } => s == "qwerty",
        _ => false
    });
}
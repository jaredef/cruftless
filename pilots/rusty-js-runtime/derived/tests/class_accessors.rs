//! Tier-Omega.5.u — class-member getter / setter compiler coverage.
//!
//! Accessor descriptors are now wired through class lowering and the
//! runtime property read/write paths: class getters run on read, setters
//! run on assignment, and Object.getOwnPropertyDescriptor exposes the
//! underlying accessor functions.

use rusty_js_runtime::{run_module, Value};

fn run(src: &str) -> Value {
    run_module(src).unwrap_or_else(|e| panic!("run failed for {:?}: {:?}", src, e))
}

// 1. Class with getter parses + compiles. Reading `new C().foo`
//    invokes the getter.
#[test]
fn t01_class_getter_compiles() {
    let src = r#"
        class C { get foo() { return 42; } }
        return new C().foo;
    "#;
    assert_eq!(run(src), Value::Number(42.0));
}

// 2. Class with setter parses + compiles. Assignment dispatches the
//    setter with `this` bound to the receiver.
#[test]
fn t02_class_setter_compiles() {
    let src = r#"
        class C { set bar(v) { this.x = v; } }
        const c = new C();
        c.bar = 7;
        return c.x;
    "#;
    assert_eq!(run(src), Value::Number(7.0));
}

// 3. Disambiguation — `get` / `set` as method names (followed by `(`),
//    not accessor modifiers.
#[test]
fn t03_get_set_as_method_names() {
    let src = r#"
        class C {
            get() { return 1; }
            set(v) { return v + 1; }
        }
        const c = new C();
        return c.get() + c.set(2);
    "#;
    assert_eq!(run(src), Value::Number(4.0));
}

// 4. Static getter lands on the constructor and runs on read.
#[test]
fn t04_static_getter() {
    let src = r#"
        class C { static get count() { return 5; } }
        return C.count;
    "#;
    assert_eq!(run(src), Value::Number(5.0));
}

// 5. Getter and setter on the same key share an accessor descriptor.
#[test]
fn t05_getter_and_setter_same_key() {
    let src = r#"
        class C {
            get x() { return 1; }
            set x(v) { return v; }
        }
        const d = Object.getOwnPropertyDescriptor(C.prototype, "x");
        return typeof d.get + "," + typeof d.set + "," + new C().x;
    "#;
    assert_eq!(run(src), Value::String(std::rc::Rc::new("function,function,1".into())));
}

use crate::register::{new_object, register_method};
use rusty_js_runtime::{Runtime, RuntimeError, Value};

pub fn install(rt: &mut Runtime) {
    if std::env::var_os("T262_TEST_PATH").is_none() {
        return;
    }

    let host = new_object(rt);
    register_method(rt, host, "detachArrayBuffer", |rt, args| {
        let id = match args.first() {
            Some(Value::Object(id)) => *id,
            _ => {
                return Err(RuntimeError::TypeError(
                    "$262.detachArrayBuffer: argument must be an ArrayBuffer".into(),
                ))
            }
        };
        rt.detach_array_buffer(id)?;
        Ok(Value::Undefined)
    });
    rt.define_global_property("$262", Value::Object(host));
}

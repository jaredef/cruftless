//! ECMA-262 §28.1.{4, 8, 9, 12, 13} — Reflect.{has, get, set, deleteProperty, ownKeys}.
//!
//! Five sections mirroring the existing Object.* patterns plus the
//! Reflect-specific "return boolean instead of throw" semantics for
//! the mutation ops.

use crate::ir::{Expr, IRFunction, IRNode, Step};
use crate::lint::SpecStepRecord;

fn v(name: &str) -> Expr {
    Expr::Var(name.to_string())
}

pub fn build_has() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "param.key".into(),
            node: IRNode::Let {
                name: "key".into(),
                value: Expr::Arg(1),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_has_via",
                args: vec![v("target"), v("key")],
            }),
        },
    ];
    IRFunction {
        spec_section: "28.1.9".into(),
        rust_name: "reflect_has".into(),
        title: "Reflect.has ( target, propertyKey )".into(),
        body,
    }
}

pub fn build_get() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "param.key".into(),
            node: IRNode::Let {
                name: "key".into(),
                value: Expr::Arg(1),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_get_via",
                args: vec![v("target"), v("key")],
            }),
        },
    ];
    IRFunction {
        spec_section: "28.1.8".into(),
        rust_name: "reflect_get".into(),
        title: "Reflect.get ( target, propertyKey [ , receiver ] )".into(),
        body,
    }
}

pub fn build_set() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "param.key".into(),
            node: IRNode::Let {
                name: "key".into(),
                value: Expr::Arg(1),
            },
        },
        Step {
            spec_step: "param.value".into(),
            node: IRNode::Let {
                name: "value".into(),
                value: Expr::Arg(2),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_set_via",
                args: vec![v("target"), v("key"), v("value")],
            }),
        },
    ];
    IRFunction {
        spec_section: "28.1.13".into(),
        rust_name: "reflect_set".into(),
        title: "Reflect.set ( target, propertyKey, V [ , receiver ] )".into(),
        body,
    }
}

pub fn build_delete_property() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "param.key".into(),
            node: IRNode::Let {
                name: "key".into(),
                value: Expr::Arg(1),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_delete_property_via",
                args: vec![v("target"), v("key")],
            }),
        },
    ];
    IRFunction {
        spec_section: "28.1.4".into(),
        rust_name: "reflect_delete_property".into(),
        title: "Reflect.deleteProperty ( target, propertyKey )".into(),
        body,
    }
}

pub fn build_own_keys() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_own_keys_via",
                args: vec![v("target")],
            }),
        },
    ];
    IRFunction {
        spec_section: "28.1.12".into(),
        rust_name: "reflect_own_keys".into(),
        title: "Reflect.ownKeys ( target )".into(),
        body,
    }
}

pub fn build_get_prototype_of() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_get_prototype_of_via",
                args: vec![v("target")],
            }),
        },
    ];
    IRFunction {
        spec_section: "28.1.7".into(),
        rust_name: "reflect_get_prototype_of".into(),
        title: "Reflect.getPrototypeOf ( target )".into(),
        body,
    }
}

pub fn build_set_prototype_of() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "param.proto".into(),
            node: IRNode::Let {
                name: "proto".into(),
                value: Expr::Arg(1),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_set_prototype_of_via",
                args: vec![v("target"), v("proto")],
            }),
        },
    ];
    IRFunction {
        spec_section: "28.1.14".into(),
        rust_name: "reflect_set_prototype_of".into(),
        title: "Reflect.setPrototypeOf ( target, proto )".into(),
        body,
    }
}

pub fn build_is_extensible() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_is_extensible_via",
                args: vec![v("target")],
            }),
        },
    ];
    IRFunction {
        spec_section: "28.1.10".into(),
        rust_name: "reflect_is_extensible".into(),
        title: "Reflect.isExtensible ( target )".into(),
        body,
    }
}

pub fn build_prevent_extensions() -> IRFunction {
    let body = vec![
        Step {
            spec_step: "param.target".into(),
            node: IRNode::Let {
                name: "target".into(),
                value: Expr::Arg(0),
            },
        },
        Step {
            spec_step: "1".into(),
            node: IRNode::Return(Expr::CallBuiltin {
                name: "reflect_prevent_extensions_via",
                args: vec![v("target")],
            }),
        },
    ];
    IRFunction {
        spec_section: "28.1.11".into(),
        rust_name: "reflect_prevent_extensions".into(),
        title: "Reflect.preventExtensions ( target )".into(),
        body,
    }
}

// ──────────────── linter records ────────────────

pub fn spec_steps_has() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["reflect_has_via"],
        throws: None,
        prose: "Return ? HasProperty(target, ToPropertyKey(propertyKey)).",
    }]
}
pub fn spec_steps_get() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["reflect_get_via"],
        throws: None,
        prose: "Return ? target.[[Get]](ToPropertyKey(propertyKey), receiver).",
    }]
}
pub fn spec_steps_set() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["reflect_set_via"],
        throws: None,
        prose: "Return ? target.[[Set]](ToPropertyKey(propertyKey), V, receiver).",
    }]
}
pub fn spec_steps_delete_property() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["reflect_delete_property_via"],
        throws: None,
        prose: "Return ? target.[[Delete]](ToPropertyKey(propertyKey)).",
    }]
}
pub fn spec_steps_own_keys() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["reflect_own_keys_via"],
        throws: None,
        prose: "Let keys be ? target.[[OwnPropertyKeys]](). Return CreateArrayFromList(keys).",
    }]
}
pub fn spec_steps_get_prototype_of() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["reflect_get_prototype_of_via"],
        throws: None,
        prose: "Return ? target.[[GetPrototypeOf]]().",
    }]
}
pub fn spec_steps_set_prototype_of() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["reflect_set_prototype_of_via"],
        throws: None,
        prose: "Return ? target.[[SetPrototypeOf]](proto).",
    }]
}
pub fn spec_steps_is_extensible() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["reflect_is_extensible_via"],
        throws: None,
        prose: "Return ? target.[[IsExtensible]]().",
    }]
}
pub fn spec_steps_prevent_extensions() -> Vec<SpecStepRecord> {
    vec![SpecStepRecord {
        step_id: "1".into(),
        abstract_ops: vec!["reflect_prevent_extensions_via"],
        throws: None,
        prose: "Return ? target.[[PreventExtensions]]().",
    }]
}

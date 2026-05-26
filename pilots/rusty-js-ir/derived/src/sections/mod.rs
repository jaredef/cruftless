//! Hand-translated ECMA-262 algorithm sections.
//!
//! Tier 1 scope per IR-DESIGN.md §7.1 — one representative section
//! (Array.prototype.map §23.1.3.20). Tier 2 will add the spec-XML parser
//! to derive these from ECMA-262 directly.

pub mod array_prototype_find; // find, findIndex, findLast, findLastIndex
pub mod array_prototype_index_search; // indexOf, includes (lastIndexOf queued)
pub mod array_prototype_iteration; // forEach, filter, every, some
pub mod array_prototype_map;
pub mod array_prototype_mutators; // push, pop, shift, unshift, reverse
pub mod array_prototype_reduce; // reduce (reduceRight queued, awaits signed-Int)
pub mod array_set_length; // §10.4.2.1 — IR-EXT 66 first higher-resolution-IR spec-step section
pub mod global_predicates; // global isFinite, isNaN (coerce; differ from Number.* versions)
pub mod json_serialize; // §25.5.2.4 SerializeJSONProperty — IR-EXT 68
pub mod math_binary_variadic; // Math.{pow,atan2} (binary) + Math.{max,min,hypot} (variadic)
pub mod math_unary; // Math.{abs,floor,ceil,round,trunc,sqrt,cbrt,sign,exp,log,sin,...}
pub mod misc_static; // Math.{imul, fround, clz32}, Array.{isArray, of}
pub mod number_prototype; // Number.prototype.{toFixed, ...}
pub mod number_static; // Number.{isFinite, isInteger, isNaN, isSafeInteger}
pub mod object_assign; // §20.1.2.1 Object.assign per-source step — IR-EXT 69
pub mod object_descriptors; // Object.{defineProperty, defineProperties, getOwnPropertyDescriptor(s), create} + Object.prototype.{__defineGetter__, __defineSetter__, __lookupGetter__, __lookupSetter__}
pub mod object_integrity; // freeze, seal, preventExtensions, hasOwn, is
pub mod object_proto_ops; // getPrototypeOf, setPrototypeOf, isExtensible, isFrozen, isSealed
pub mod object_prototype; // Object.prototype.{toString, hasOwnProperty, valueOf, propertyIsEnumerable, isPrototypeOf, toLocaleString}
pub mod object_static; // Object.keys, Object.values, Object.entries
pub mod promise_static; // Promise.resolve, Promise.reject
pub mod reflect_static; // Reflect.{has, get, set, deleteProperty, ownKeys}
pub mod string_prototype; // String.prototype.{charAt, charCodeAt, concat, ...}
pub mod to_primitive; // §7.1.1 ToPrimitive resolver-instance — IR-EXT 72

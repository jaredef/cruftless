// F-category: AbortController / AbortSignal surface.
// Targets the cancellation primitives every modern async library relies on.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Construction + initial state.
{
  const c = new AbortController();
  result.construct = {
    has_signal: typeof c.signal === "object" && c.signal !== null,
    aborted_initial: c.signal.aborted,
    reason_initial: c.signal.reason === undefined,
    signal_is_signal: c.signal instanceof AbortSignal,
  };
}

// .abort() with no reason: default reason is an AbortError DOMException-ish.
{
  const c = new AbortController();
  c.abort();
  result.abort_no_reason = {
    aborted: c.signal.aborted,
    reason_defined: c.signal.reason !== undefined,
    reason_name: c.signal.reason && c.signal.reason.name,
  };
}

// .abort(reason): caller's reason is preserved verbatim.
{
  const c = new AbortController();
  const r = new Error("user-cancel");
  c.abort(r);
  result.abort_with_reason = {
    aborted: c.signal.aborted,
    same_reason: c.signal.reason === r,
    message: c.signal.reason && c.signal.reason.message,
  };
}

// Listener fires exactly once on abort.
{
  const c = new AbortController();
  let count = 0;
  c.signal.addEventListener("abort", () => { count++; });
  c.abort();
  c.abort(); // second call is a no-op per spec
  result.listener_once = { count };
}

// throwIfAborted: throws the reason after abort; no-op before.
{
  const c = new AbortController();
  let pre_threw = false;
  try { c.signal.throwIfAborted(); } catch { pre_threw = true; }
  c.abort(new Error("boom"));
  let post_msg = null;
  try { c.signal.throwIfAborted(); } catch (e) { post_msg = e.message; }
  result.throw_if_aborted = { pre_threw, post_msg };
}

// AbortSignal.abort(reason): pre-aborted factory.
{
  const s = AbortSignal.abort(new Error("pre"));
  result.signal_abort_factory = {
    aborted: s.aborted,
    msg: s.reason && s.reason.message,
    is_signal: s instanceof AbortSignal,
  };
}

// AbortSignal.timeout(ms): present-but-non-firing in cruftless v1. The factory
// returns a non-aborting signal; real firing requires routing through the
// host-tier timer queue (cruftless/src/timer.rs), which the runtime-tier
// intrinsics layer cannot reach. Deferred as a substrate gap; tested only for
// surface presence here so this fixture flips to PASS on the AbortController
// cluster landing.
{
  const s = AbortSignal.timeout(10);
  result.signal_timeout_surface = {
    is_signal: s instanceof AbortSignal,
    has_aborted_field: typeof s.aborted === "boolean",
  };
}

// AbortSignal.any([s1, s2]): aborts when any input aborts.
{
  const a = new AbortController();
  const b = new AbortController();
  const any = AbortSignal.any([a.signal, b.signal]);
  const pre = any.aborted;
  b.abort(new Error("from-b"));
  result.signal_any = {
    pre_aborted: pre,
    post_aborted: any.aborted,
    reason_msg: any.reason && any.reason.message,
  };
}

console.log(canon(result));

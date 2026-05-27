// F-category: SharedArrayBuffer + Atomics deterministic surface.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// SharedArrayBuffer presence.
{
  result.sab_present = typeof SharedArrayBuffer === "function";
  result.atomics_present = typeof Atomics === "object";
}

if (typeof SharedArrayBuffer === "function" && typeof Atomics === "object") {

  // Basic SharedArrayBuffer + Int32Array.
  {
    const sab = new SharedArrayBuffer(16);
    const view = new Int32Array(sab);
    result.sab_shape = {
      byte_length: sab.byteLength,
      view_length: view.length,
      is_shared: view.buffer instanceof SharedArrayBuffer,
    };
  }

  // Atomics.store / Atomics.load.
  {
    const sab = new SharedArrayBuffer(16);
    const view = new Int32Array(sab);
    Atomics.store(view, 0, 42);
    Atomics.store(view, 1, -1);
    result.store_load = {
      v0: Atomics.load(view, 0),
      v1: Atomics.load(view, 1),
      store_returns: Atomics.store(view, 2, 99),
    };
  }

  // Atomics.add / Atomics.sub: return old value.
  {
    const sab = new SharedArrayBuffer(16);
    const view = new Int32Array(sab);
    Atomics.store(view, 0, 10);
    const old_add = Atomics.add(view, 0, 5);
    const after_add = Atomics.load(view, 0);
    const old_sub = Atomics.sub(view, 0, 3);
    const after_sub = Atomics.load(view, 0);
    result.add_sub = { old_add, after_add, old_sub, after_sub };
  }

  // Atomics.and / Atomics.or / Atomics.xor: return old value.
  {
    const sab = new SharedArrayBuffer(16);
    const view = new Int32Array(sab);
    Atomics.store(view, 0, 0b1111);
    const old_and = Atomics.and(view, 0, 0b1010);
    const after_and = Atomics.load(view, 0);

    Atomics.store(view, 1, 0b1010);
    const old_or = Atomics.or(view, 1, 0b0101);
    const after_or = Atomics.load(view, 1);

    Atomics.store(view, 2, 0b1100);
    const old_xor = Atomics.xor(view, 2, 0b1010);
    const after_xor = Atomics.load(view, 2);

    result.bitwise = { old_and, after_and, old_or, after_or, old_xor, after_xor };
  }

  // Atomics.exchange: return old value, store new.
  {
    const sab = new SharedArrayBuffer(8);
    const view = new Int32Array(sab);
    Atomics.store(view, 0, 100);
    const old = Atomics.exchange(view, 0, 200);
    const current = Atomics.load(view, 0);
    result.exchange = { old, current };
  }

  // Atomics.compareExchange: CAS semantics.
  {
    const sab = new SharedArrayBuffer(8);
    const view = new Int32Array(sab);
    Atomics.store(view, 0, 10);

    const old1 = Atomics.compareExchange(view, 0, 10, 20);
    const after1 = Atomics.load(view, 0);

    const old2 = Atomics.compareExchange(view, 0, 999, 30);
    const after2 = Atomics.load(view, 0);

    result.cas = {
      success_old: old1, success_after: after1,
      fail_old: old2, fail_after: after2,
    };
  }

  // Atomics.isLockFree.
  {
    result.is_lock_free = {
      size_1: Atomics.isLockFree(1),
      size_2: Atomics.isLockFree(2),
      size_4: Atomics.isLockFree(4),
      size_8: Atomics.isLockFree(8),
      size_3: Atomics.isLockFree(3),
    };
  }

  // Atomics.wait / Atomics.notify: shape presence.
  {
    result.wait_notify = {
      wait_present: typeof Atomics.wait === "function",
      notify_present: typeof Atomics.notify === "function",
      waitAsync_present: typeof Atomics.waitAsync === "function",
    };
  }

  // Atomics.wait with timeout 0: returns "timed-out" immediately (no blocking).
  {
    const sab = new SharedArrayBuffer(8);
    const view = new Int32Array(sab);
    Atomics.store(view, 0, 0);
    let wait_result = null;
    try {
      wait_result = Atomics.wait(view, 0, 0, 0);
    } catch (e) {
      wait_result = "error:" + e.constructor.name;
    }
    result.wait_timeout_zero = { wait_result };
  }

  // Atomics.wait with value mismatch: returns "not-equal" immediately.
  {
    const sab = new SharedArrayBuffer(8);
    const view = new Int32Array(sab);
    Atomics.store(view, 0, 42);
    let wait_result = null;
    try {
      wait_result = Atomics.wait(view, 0, 999, 0);
    } catch (e) {
      wait_result = "error:" + e.constructor.name;
    }
    result.wait_not_equal = { wait_result };
  }

  // Atomics.notify returns count of woken agents (0 when none waiting).
  {
    const sab = new SharedArrayBuffer(8);
    const view = new Int32Array(sab);
    const woken = Atomics.notify(view, 0, 1);
    result.notify_none = { woken };
  }

  // TypedArray type variations.
  {
    const sab = new SharedArrayBuffer(16);
    const i8 = new Int8Array(sab);
    const u8 = new Uint8Array(sab);
    const i16 = new Int16Array(sab);
    const u32 = new Uint32Array(sab);

    Atomics.store(i8, 0, 127);
    Atomics.store(u8, 4, 255);
    Atomics.store(i16, 4, -1);
    Atomics.store(u32, 3, 4294967295);

    result.typed_variations = {
      i8: Atomics.load(i8, 0),
      u8: Atomics.load(u8, 4),
      i16: Atomics.load(i16, 4),
      u32: Atomics.load(u32, 3),
    };
  }

}

console.log(canon(result));

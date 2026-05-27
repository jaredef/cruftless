# typed-array-resizable-buffer-indexed-access — Diff-Prod Analysis

| Fixture | Status | Relevance |
|---|---|---|
| typed-arrays | PASS | TypedArray indexed access is the observable surface; RAB changes the backing store model |
| arraybuffer-operations | FAIL | ArrayBuffer operations (including resize) are the direct substrate this locale must implement |
| dataview-methods | FAIL | DataView over ResizableArrayBuffer is part of the same backing-store substrate |

This locale targets the shared buffer/view substrate for ResizableArrayBuffer-backed TypedArray indexed access. The arraybuffer-operations FAIL is directly relevant: ArrayBuffer.prototype.resize is a missing method that this locale needs to implement. The dataview-methods FAIL also connects because DataView views over resizable buffers share the same backing-store representation gap. The typed-arrays PASS suggests the non-resizable TypedArray path works, isolating the gap to the RAB substrate (no first-class backing buffer object, no resize method).

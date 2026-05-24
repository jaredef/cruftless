# TCC failure-frequency table — 2026-05-24

Total files measured: 374. OK: 259 (69.3% parse-success).

Rows ranked by file count. Each row's `structural tag` names a TS feature / shape concept; an example file is given so the failure can be inspected. Sub-locale priority order is set by this table.

| Rank | Structural tag | Files | Example | Sample message |
|---:|---|---:|---|---|
| 1 | `method-return-annotation` | 37 | `pino/test/fixtures/ts/transport-worker.ts` | `ParseError { span: Span { start: 56, end: 57 }, message: "unexpected token in ex` |
| 2 | `generic-call` | 27 | `rxjs/src/internal/firstValueFrom.ts` | `ParseError { span: Span { start: 312, end: 313 }, message: "expected `LBrace`" }` |
| 3 | `readonly-modifier` | 9 | `ajv/lib/compile/codegen/code.ts` | `ParseError { span: Span { start: 375, end: 376 }, message: "expected class membe` |
| 4 | `uncategorized-unexpected-token` | 7 | `ajv/lib/compile/validate/boolSchema.ts` | `ParseError { span: Span { start: 1072, end: 1073 }, message: "unexpected token i` |
| 5 | `template-literal-type` | 5 | `ajv/lib/compile/errors.ts` | `ParseError { span: Span { start: 2951, end: 2952 }, message: "expected template ` |
| 6 | `import-export-type` | 5 | `ajv/lib/compile/jtd/parse.ts` | `ParseError { span: Span { start: 12, end: 15 }, message: "expected `from`" }` |
| 7 | `keyof-type` | 2 | `rxjs/src/internal/operators/distinctUntilKeyChanged.ts` | `ParseError { span: Span { start: 239, end: 240 }, message: "expected `LBrace`" }` |
| 8 | `other: ParseError { span: Span { start: 2982, e` | 1 | `rxjs/src/internal/observable/timer.ts` | `ParseError { span: Span { start: 2982, end: 2983 }, message: "expected `LBrace`"` |
| 9 | `other: ParseError { span: Span { start: 192, en` | 1 | `rxjs/src/internal/observable/range.ts` | `ParseError { span: Span { start: 192, end: 193 }, message: "expected `LBrace`" }` |
| 10 | `other: ParseError { span: Span { start: 1592, e` | 1 | `ajv/lib/vocabularies/core/ref.ts` | `ParseError { span: Span { start: 1592, end: 1593 }, message: "expected `Colon`" ` |
| 11 | `other: ParseError { span: Span { start: 2146, e` | 1 | `rxjs/src/internal/Notification.ts` | `ParseError { span: Span { start: 2146, end: 2147 }, message: "expected `LBrace`"` |
| 12 | `lex-unterminated` | 1 | `rxjs/src/internal/testing/TestScheduler.ts` | `strip: ts-strip @11905: lex: LexError { kind: UnterminatedRegex, span: Span { st` |
| 13 | `other: ParseError { span: Span { start: 629, en` | 1 | `ajv/lib/runtime/ucs2length.ts` | `ParseError { span: Span { start: 629, end: 629 }, message: "expected `RBrace`" }` |
| 14 | `other: ParseError { span: Span { start: 196, en` | 1 | `pino/test/types/pino.test-d.ts` | `ParseError { span: Span { start: 196, end: 197 }, message: "expected `from`" }` |
| 15 | `as-const` | 1 | `rxjs/src/internal/ajax/ajax.ts` | `ParseError { span: Span { start: 15755, end: 15756 }, message: "expected templat` |
| 16 | `other: ParseError { span: Span { start: 285, en` | 1 | `rxjs/src/internal/operators/findIndex.ts` | `ParseError { span: Span { start: 285, end: 286 }, message: "expected `LBrace`" }` |
| 17 | `other: ParseError { span: Span { start: 329, en` | 1 | `rxjs/src/internal/operators/every.ts` | `ParseError { span: Span { start: 329, end: 330 }, message: "expected `LBrace`" }` |
| 18 | `other: ParseError { span: Span { start: 1862, e` | 1 | `ajv/lib/vocabularies/jtd/ref.ts` | `ParseError { span: Span { start: 1862, end: 1863 }, message: "expected `Colon`" ` |
| 19 | `enum` | 1 | `ajv/lib/vocabularies/jtd/properties.ts` | `ParseError { span: Span { start: 609, end: 610 }, message: "unexpected token in ` |
| 20 | `other: ParseError { span: Span { start: 520, en` | 1 | `rxjs/src/internal/observable/of.ts` | `ParseError { span: Span { start: 520, end: 521 }, message: "expected `LBrace`" }` |
| 21 | `other: ParseError { span: Span { start: 421, en` | 1 | `rxjs/src/internal/operators/mergeMapTo.ts` | `ParseError { span: Span { start: 421, end: 422 }, message: "expected `LBrace`" }` |
| 22 | `other: ParseError { span: Span { start: 973, en` | 1 | `rxjs/src/internal/ajax/getXHRResponse.ts` | `ParseError { span: Span { start: 973, end: 974 }, message: "expected `Colon`" }` |
| 23 | `other: ParseError { span: Span { start: 9302, e` | 1 | `rxjs/src/internal/observable/dom/WebSocketSubject.ts` | `ParseError { span: Span { start: 9302, end: 9303 }, message: "expected `Colon`" ` |
| 24 | `other: ParseError { span: Span { start: 422, en` | 1 | `rxjs/src/internal/operators/expand.ts` | `ParseError { span: Span { start: 422, end: 423 }, message: "expected `LBrace`" }` |
| 25 | `other: ParseError { span: Span { start: 12299, ` | 1 | `ajv/lib/compile/index.ts` | `ParseError { span: Span { start: 12299, end: 12299 }, message: "expected `from`"` |
| 26 | `other: ParseError { span: Span { start: 2815, e` | 1 | `rxjs/src/internal/observable/generate.ts` | `ParseError { span: Span { start: 2815, end: 2816 }, message: "expected `LBrace`"` |
| 27 | `uncategorized-class-member` | 1 | `ajv/lib/standalone/instance.ts` | `ParseError { span: Span { start: 202, end: 203 }, message: "expected class membe` |
| 28 | `access-modifier` | 1 | `rxjs/src/internal/scheduler/VirtualTimeScheduler.ts` | `ParseError { span: Span { start: 1777, end: 1778 }, message: "unexpected token i` |
| 29 | `other: ParseError { span: Span { start: 7289, e` | 1 | `ajv/lib/types/index.ts` | `ParseError { span: Span { start: 7289, end: 7289 }, message: "expected `from`" }` |
| 30 | `other: ParseError { span: Span { start: 178, en` | 1 | `pino/test/types/pino-import.test-d.ts` | `ParseError { span: Span { start: 178, end: 179 }, message: "expected `from`" }` |

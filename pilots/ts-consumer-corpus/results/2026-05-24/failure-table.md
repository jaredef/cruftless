# TCC failure-frequency table — 2026-05-24

Total files measured: 374. OK: 224 (59.9% parse-success).

Rows ranked by file count. Each row's `structural tag` names a TS feature / shape concept; an example file is given so the failure can be inspected. Sub-locale priority order is set by this table.

| Rank | Structural tag | Files | Example | Sample message |
|---:|---|---:|---|---|
| 1 | `method-return-annotation` | 41 | `ajv/lib/ajv.ts` | `ParseError { span: Span { start: 941, end: 942 }, message: "expected `Colon`" }` |
| 2 | `generic-call` | 40 | `rxjs/src/internal/Observable.ts` | `ParseError { span: Span { start: 2079, end: 2080 }, message: "unexpected token i` |
| 3 | `readonly-modifier` | 9 | `ajv/lib/compile/codegen/code.ts` | `ParseError { span: Span { start: 375, end: 376 }, message: "expected class membe` |
| 4 | `template-literal-type` | 7 | `ajv/lib/compile/errors.ts` | `ParseError { span: Span { start: 2951, end: 2952 }, message: "expected template ` |
| 5 | `uncategorized-unexpected-token` | 6 | `ajv/lib/compile/validate/boolSchema.ts` | `ParseError { span: Span { start: 1072, end: 1073 }, message: "unexpected token i` |
| 6 | `import-export-type` | 5 | `ajv/lib/compile/jtd/parse.ts` | `ParseError { span: Span { start: 12, end: 15 }, message: "expected `from`" }` |
| 7 | `access-modifier` | 3 | `rxjs/src/internal/Scheduler.ts` | `ParseError { span: Span { start: 2330, end: 2331 }, message: "expected class mem` |
| 8 | `keyof-type` | 2 | `rxjs/src/internal/operators/distinctUntilKeyChanged.ts` | `ParseError { span: Span { start: 239, end: 240 }, message: "expected `LBrace`" }` |
| 9 | `other: ParseError { span: Span { start: 1614, e` | 1 | `rxjs/src/internal/operators/take.ts` | `ParseError { span: Span { start: 1614, end: 1615 }, message: "expected `Colon`" ` |
| 10 | `other: ParseError { span: Span { start: 2815, e` | 1 | `rxjs/src/internal/observable/generate.ts` | `ParseError { span: Span { start: 2815, end: 2816 }, message: "expected `LBrace`"` |
| 11 | `enum` | 1 | `ajv/lib/vocabularies/jtd/properties.ts` | `ParseError { span: Span { start: 609, end: 610 }, message: "unexpected token in ` |
| 12 | `other: ParseError { span: Span { start: 1862, e` | 1 | `ajv/lib/vocabularies/jtd/ref.ts` | `ParseError { span: Span { start: 1862, end: 1863 }, message: "expected `Colon`" ` |
| 13 | `other: ParseError { span: Span { start: 2146, e` | 1 | `rxjs/src/internal/Notification.ts` | `ParseError { span: Span { start: 2146, end: 2147 }, message: "expected `LBrace`"` |
| 14 | `other: ParseError { span: Span { start: 2982, e` | 1 | `rxjs/src/internal/observable/timer.ts` | `ParseError { span: Span { start: 2982, end: 2983 }, message: "expected `LBrace`"` |
| 15 | `other: ParseError { span: Span { start: 973, en` | 1 | `rxjs/src/internal/ajax/getXHRResponse.ts` | `ParseError { span: Span { start: 973, end: 974 }, message: "expected `Colon`" }` |
| 16 | `other: ParseError { span: Span { start: 629, en` | 1 | `ajv/lib/runtime/ucs2length.ts` | `ParseError { span: Span { start: 629, end: 629 }, message: "expected `RBrace`" }` |
| 17 | `other: ParseError { span: Span { start: 2071, e` | 1 | `rxjs/src/internal/observable/empty.ts` | `ParseError { span: Span { start: 2071, end: 2072 }, message: "expected `Colon`" ` |
| 18 | `uncategorized-class-member` | 1 | `ajv/lib/standalone/instance.ts` | `ParseError { span: Span { start: 202, end: 203 }, message: "expected class membe` |
| 19 | `other: ParseError { span: Span { start: 4665, e` | 1 | `rxjs/src/internal/ajax/AjaxResponse.ts` | `ParseError { span: Span { start: 4665, end: 4666 }, message: "expected `Colon`" ` |
| 20 | `other: ParseError { span: Span { start: 520, en` | 1 | `rxjs/src/internal/observable/of.ts` | `ParseError { span: Span { start: 520, end: 521 }, message: "expected `LBrace`" }` |
| 21 | `other: ParseError { span: Span { start: 2125, e` | 1 | `rxjs/src/internal/operators/skipLast.ts` | `ParseError { span: Span { start: 2125, end: 2126 }, message: "expected `Colon`" ` |
| 22 | `other: ParseError { span: Span { start: 2703, e` | 1 | `rxjs/src/internal/observable/using.ts` | `ParseError { span: Span { start: 2703, end: 2704 }, message: "expected `Colon`" ` |
| 23 | `other: ParseError { span: Span { start: 4801, e` | 1 | `rxjs/src/internal/scheduler/AsyncAction.ts` | `ParseError { span: Span { start: 4801, end: 4802 }, message: "expected `Colon`" ` |
| 24 | `other: ParseError { span: Span { start: 421, en` | 1 | `rxjs/src/internal/operators/mergeMapTo.ts` | `ParseError { span: Span { start: 421, end: 422 }, message: "expected `LBrace`" }` |
| 25 | `other: ParseError { span: Span { start: 2723, e` | 1 | `rxjs/src/internal/operators/distinct.ts` | `ParseError { span: Span { start: 2723, end: 2724 }, message: "expected `Colon`" ` |
| 26 | `other: ParseError { span: Span { start: 1374, e` | 1 | `rxjs/src/internal/operators/min.ts` | `ParseError { span: Span { start: 1374, end: 1375 }, message: "expected `Colon`" ` |
| 27 | `other: ParseError { span: Span { start: 1373, e` | 1 | `rxjs/src/internal/operators/max.ts` | `ParseError { span: Span { start: 1373, end: 1374 }, message: "expected `Colon`" ` |
| 28 | `other: ParseError { span: Span { start: 2535, e` | 1 | `rxjs/src/internal/observable/iif.ts` | `ParseError { span: Span { start: 2535, end: 2536 }, message: "expected `Colon`" ` |
| 29 | `other: ParseError { span: Span { start: 1592, e` | 1 | `ajv/lib/vocabularies/core/ref.ts` | `ParseError { span: Span { start: 1592, end: 1593 }, message: "expected `Colon`" ` |
| 30 | `other: ParseError { span: Span { start: 178, en` | 1 | `pino/test/types/pino-import.test-d.ts` | `ParseError { span: Span { start: 178, end: 179 }, message: "expected `from`" }` |

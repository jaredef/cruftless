# TCC failure-frequency table — 2026-05-24

Total files measured: 374. OK: 306 (81.8% parse-success).

Rows ranked by file count. Each row's `structural tag` names a TS feature / shape concept; an example file is given so the failure can be inspected. Sub-locale priority order is set by this table.

| Rank | Structural tag | Files | Example | Sample message |
|---:|---|---:|---|---|
| 1 | `method-return-annotation` | 15 | `rxjs/src/internal/observable/combineLatest.ts` | `ParseError { span: Span { start: 3861, end: 3862 }, message: "expected `LParen`"` |
| 2 | `generic-call` | 14 | `rxjs/src/internal/observable/dom/animationFrames.ts` | `ParseError { span: Span { start: 3161, end: 3162 }, message: "expected `RBrace`"` |
| 3 | `uncategorized-unexpected-token` | 8 | `rxjs/src/internal/Subscriber.ts` | `ParseError { span: Span { start: 8949, end: 8950 }, message: "unexpected token i` |
| 4 | `import-export-type` | 5 | `ajv/lib/compile/jtd/parse.ts` | `ParseError { span: Span { start: 12, end: 15 }, message: "expected `from`" }` |
| 5 | `template-literal-type` | 4 | `ajv/lib/compile/errors.ts` | `ParseError { span: Span { start: 2951, end: 2952 }, message: "expected template ` |
| 6 | `other: ParseError { span: Span { start: 973, en` | 1 | `rxjs/src/internal/ajax/getXHRResponse.ts` | `ParseError { span: Span { start: 973, end: 974 }, message: "expected `Colon`" }` |
| 7 | `other: ParseError { span: Span { start: 178, en` | 1 | `pino/test/types/pino-import.test-d.ts` | `ParseError { span: Span { start: 178, end: 179 }, message: "expected `from`" }` |
| 8 | `other: ParseError { span: Span { start: 520, en` | 1 | `rxjs/src/internal/observable/of.ts` | `ParseError { span: Span { start: 520, end: 521 }, message: "expected `LBrace`" }` |
| 9 | `lex-unterminated` | 1 | `rxjs/src/internal/testing/TestScheduler.ts` | `strip: ts-strip @11905: lex: LexError { kind: UnterminatedRegex, span: Span { st` |
| 10 | `other: ParseError { span: Span { start: 11874, ` | 1 | `rxjs/src/internal/observable/generate.ts` | `ParseError { span: Span { start: 11874, end: 11875 }, message: "expected `RParen` |
| 11 | `uncategorized-class-member` | 1 | `ajv/lib/standalone/instance.ts` | `ParseError { span: Span { start: 202, end: 203 }, message: "expected class membe` |
| 12 | `enum` | 1 | `ajv/lib/vocabularies/jtd/properties.ts` | `ParseError { span: Span { start: 609, end: 610 }, message: "unexpected token in ` |
| 13 | `abstract-modifier` | 1 | `ajv/lib/compile/codegen/code.ts` | `ParseError { span: Span { start: 193, end: 194 }, message: "expected `LBrace`" }` |
| 14 | `other: ParseError { span: Span { start: 7289, e` | 1 | `ajv/lib/types/index.ts` | `ParseError { span: Span { start: 7289, end: 7289 }, message: "expected `from`" }` |
| 15 | `other: ParseError { span: Span { start: 12299, ` | 1 | `ajv/lib/compile/index.ts` | `ParseError { span: Span { start: 12299, end: 12299 }, message: "expected `from`"` |
| 16 | `other: ParseError { span: Span { start: 6353, e` | 1 | `rxjs/src/internal/operators/distinctUntilChanged.ts` | `ParseError { span: Span { start: 6353, end: 6354 }, message: "expected `RParen`"` |
| 17 | `other: ParseError { span: Span { start: 1592, e` | 1 | `ajv/lib/vocabularies/core/ref.ts` | `ParseError { span: Span { start: 1592, end: 1593 }, message: "expected `Colon`" ` |
| 18 | `other: ParseError { span: Span { start: 422, en` | 1 | `rxjs/src/internal/operators/expand.ts` | `ParseError { span: Span { start: 422, end: 423 }, message: "expected `LBrace`" }` |
| 19 | `other: ParseError { span: Span { start: 3426, e` | 1 | `rxjs/src/internal/operators/bufferTime.ts` | `ParseError { span: Span { start: 3426, end: 3427 }, message: "expected `RParen`"` |
| 20 | `other: ParseError { span: Span { start: 421, en` | 1 | `rxjs/src/internal/operators/mergeMapTo.ts` | `ParseError { span: Span { start: 421, end: 422 }, message: "expected `LBrace`" }` |
| 21 | `other: ParseError { span: Span { start: 1862, e` | 1 | `ajv/lib/vocabularies/jtd/ref.ts` | `ParseError { span: Span { start: 1862, end: 1863 }, message: "expected `Colon`" ` |
| 22 | `readonly-modifier` | 1 | `rxjs/src/internal/operators/startWith.ts` | `ParseError { span: Span { start: 977, end: 978 }, message: "expected `LBrace`" }` |
| 23 | `other: ParseError { span: Span { start: 7543, e` | 1 | `rxjs/src/internal/operators/tap.ts` | `ParseError { span: Span { start: 7543, end: 7544 }, message: "expected `RBrace`"` |
| 24 | `other: ParseError { span: Span { start: 9302, e` | 1 | `rxjs/src/internal/observable/dom/WebSocketSubject.ts` | `ParseError { span: Span { start: 9302, end: 9303 }, message: "expected `Colon`" ` |
| 25 | `as-const` | 1 | `rxjs/src/internal/ajax/ajax.ts` | `ParseError { span: Span { start: 15755, end: 15756 }, message: "expected templat` |
| 26 | `other: ParseError { span: Span { start: 196, en` | 1 | `pino/test/types/pino.test-d.ts` | `ParseError { span: Span { start: 196, end: 197 }, message: "expected `from`" }` |
| 27 | `other: ParseError { span: Span { start: 7826, e` | 1 | `rxjs/src/internal/Notification.ts` | `ParseError { span: Span { start: 7826, end: 7827 }, message: "expected `Colon`" ` |

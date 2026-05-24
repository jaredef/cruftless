# TCC failure-frequency table — 2026-05-24

Total files measured: 374. OK: 361 (96.5% parse-success).

Rows ranked by file count. Each row's `structural tag` names a TS feature / shape concept; an example file is given so the failure can be inspected. Sub-locale priority order is set by this table.

| Rank | Structural tag | Files | Example | Sample message |
|---:|---|---:|---|---|
| 1 | `generic-call` | 2 | `rxjs/src/internal/observable/dom/animationFrames.ts` | `ParseError { span: Span { start: 3161, end: 3162 }, message: "expected `RBrace`"` |
| 2 | `lex-unterminated` | 1 | `rxjs/src/internal/testing/TestScheduler.ts` | `strip: ts-strip @11905: lex: LexError { kind: UnterminatedRegex, span: Span { st` |
| 3 | `other: ParseError { span: Span { start: 11874, ` | 1 | `rxjs/src/internal/observable/generate.ts` | `ParseError { span: Span { start: 11874, end: 11875 }, message: "expected `RParen` |
| 4 | `other: ParseError { span: Span { start: 12833, ` | 1 | `rxjs/src/internal/operators/timeout.ts` | `ParseError { span: Span { start: 12833, end: 12834 }, message: "expected `Colon`` |
| 5 | `as-const` | 1 | `rxjs/src/internal/ajax/ajax.ts` | `ParseError { span: Span { start: 15755, end: 15756 }, message: "expected templat` |
| 6 | `uncategorized-class-member` | 1 | `ajv/lib/standalone/instance.ts` | `ParseError { span: Span { start: 202, end: 203 }, message: "expected class membe` |
| 7 | `other: ParseError { span: Span { start: 9302, e` | 1 | `rxjs/src/internal/observable/dom/WebSocketSubject.ts` | `ParseError { span: Span { start: 9302, end: 9303 }, message: "expected `Colon`" ` |
| 8 | `other: ParseError { span: Span { start: 178, en` | 1 | `pino/test/types/pino-import.test-d.ts` | `ParseError { span: Span { start: 178, end: 179 }, message: "expected `from`" }` |
| 9 | `other: ParseError { span: Span { start: 6353, e` | 1 | `rxjs/src/internal/operators/distinctUntilChanged.ts` | `ParseError { span: Span { start: 6353, end: 6354 }, message: "expected `RParen`"` |
| 10 | `other: ParseError { span: Span { start: 196, en` | 1 | `pino/test/types/pino.test-d.ts` | `ParseError { span: Span { start: 196, end: 197 }, message: "expected `from`" }` |
| 11 | `other: ParseError { span: Span { start: 520, en` | 1 | `rxjs/src/internal/observable/of.ts` | `ParseError { span: Span { start: 520, end: 521 }, message: "expected `LBrace`" }` |
| 12 | `other: ParseError { span: Span { start: 1087, e` | 1 | `rxjs/src/internal/ajax/getXHRResponse.ts` | `ParseError { span: Span { start: 1087, end: 1088 }, message: "expected `Colon`" ` |

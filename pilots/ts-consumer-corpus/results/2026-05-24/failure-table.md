# TCC failure-frequency table — 2026-05-24

Total files measured: 374. OK: 368 (98.4% parse-success).

Rows ranked by file count. Each row's `structural tag` names a TS feature / shape concept; an example file is given so the failure can be inspected. Sub-locale priority order is set by this table.

| Rank | Structural tag | Files | Example | Sample message |
|---:|---|---:|---|---|
| 1 | `generic-call` | 1 | `rxjs/src/internal/observable/dom/animationFrames.ts` | `ParseError { span: Span { start: 3161, end: 3162 }, message: "expected `RBrace`"` |
| 2 | `other: ParseError { span: Span { start: 12833, ` | 1 | `rxjs/src/internal/operators/timeout.ts` | `ParseError { span: Span { start: 12833, end: 12834 }, message: "expected `Colon`` |
| 3 | `other: ParseError { span: Span { start: 178, en` | 1 | `pino/test/types/pino-import.test-d.ts` | `ParseError { span: Span { start: 178, end: 179 }, message: "expected `from`" }` |
| 4 | `lex-unterminated` | 1 | `rxjs/src/internal/testing/TestScheduler.ts` | `strip: ts-strip @11905: lex: LexError { kind: UnterminatedRegex, span: Span { st` |
| 5 | `as-const` | 1 | `rxjs/src/internal/ajax/ajax.ts` | `ParseError { span: Span { start: 15755, end: 15756 }, message: "expected templat` |
| 6 | `other: ParseError { span: Span { start: 196, en` | 1 | `pino/test/types/pino.test-d.ts` | `ParseError { span: Span { start: 196, end: 197 }, message: "expected `from`" }` |

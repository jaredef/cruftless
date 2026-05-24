# TCC failure-frequency table — 2026-05-24

Total files measured: 374. OK: 141 (37.7% parse-success).

Rows ranked by file count. Each row's `structural tag` names a TS feature / shape concept; an example file is given so the failure can be inspected. Sub-locale priority order is set by this table.

| Rank | Structural tag | Files | Example | Sample message |
|---:|---|---:|---|---|
| 1 | `template-literal-type` | 48 | `ajv/lib/compile/codegen/code.ts` | `strip: ts-strip @3127: lex: LexError { kind: UnterminatedString, span: Span { st` |
| 2 | `method-return-annotation` | 46 | `ajv/lib/compile/codegen/index.ts` | `strip: ts-strip @23051: lex: LexError { kind: UnterminatedTemplate, span: Span {` |
| 3 | `generic-call` | 37 | `ajv/lib/core.ts` | `strip: ts-strip @12168: lex: LexError { kind: UnterminatedString, span: Span { s` |
| 4 | `uncategorized-unexpected-token` | 20 | `rxjs/src/internal/operators/audit.ts` | `ParseError { span: Span { start: 2263, end: 2264 }, message: "unexpected token i` |
| 5 | `readonly-modifier` | 11 | `ajv/lib/compile/codegen/scope.ts` | `strip: ts-strip @2928: lex: LexError { kind: InvalidIdentifier, span: Span { sta` |
| 6 | `access-modifier` | 10 | `rxjs/src/internal/Subject.ts` | `ParseError { span: Span { start: 778, end: 779 }, message: "expected `LBrace`" }` |
| 7 | `decorator` | 9 | `rxjs/src/internal/AsyncSubject.ts` | `ParseError { span: Span { start: 298, end: 299 }, message: "expected `LBrace`" }` |
| 8 | `lex-unterminated` | 7 | `ajv/lib/compile/index.ts` | `strip: ts-strip @12299: lex: LexError { kind: UnterminatedTemplate, span: Span {` |
| 9 | `lex-invalid-identifier` | 4 | `ajv/lib/runtime/parseJson.ts` | `strip: ts-strip @29: lex: LexError { kind: InvalidIdentifier, span: Span { start` |
| 10 | `import-export-type` | 2 | `ajv/lib/refs/json-schema-2019-09/index.ts` | `ParseError { span: Span { start: 12, end: 15 }, message: "expected `from`" }` |
| 11 | `keyof-type` | 2 | `rxjs/src/internal/operators/distinctUntilKeyChanged.ts` | `ParseError { span: Span { start: 239, end: 240 }, message: "expected `LBrace`" }` |
| 12 | `other: ParseError { span: Span { start: 629, en` | 1 | `ajv/lib/runtime/ucs2length.ts` | `ParseError { span: Span { start: 629, end: 629 }, message: "expected `RBrace`" }` |
| 13 | `other: ParseError { span: Span { start: 1570, e` | 1 | `rxjs/src/internal/operators/switchScan.ts` | `ParseError { span: Span { start: 1570, end: 1571 }, message: "expected `RParen`"` |
| 14 | `other: ParseError { span: Span { start: 1614, e` | 1 | `rxjs/src/internal/operators/take.ts` | `ParseError { span: Span { start: 1614, end: 1615 }, message: "expected `Colon`" ` |
| 15 | `other: ParseError { span: Span { start: 2882, e` | 1 | `rxjs/src/internal/observable/dom/animationFrames.ts` | `ParseError { span: Span { start: 2882, end: 2883 }, message: "expected `Colon`" ` |
| 16 | `other: ParseError { span: Span { start: 4985, e` | 1 | `rxjs/src/internal/observable/dom/WebSocketSubject.ts` | `ParseError { span: Span { start: 4985, end: 4986 }, message: "expected `RParen`"` |
| 17 | `other: ParseError { span: Span { start: 2790, e` | 1 | `rxjs/src/internal/ajax/errors.ts` | `ParseError { span: Span { start: 2790, end: 2791 }, message: "expected `RParen`"` |
| 18 | `other: ParseError { span: Span { start: 973, en` | 1 | `rxjs/src/internal/ajax/getXHRResponse.ts` | `ParseError { span: Span { start: 973, end: 974 }, message: "expected `Colon`" }` |
| 19 | `other: ParseError { span: Span { start: 2071, e` | 1 | `rxjs/src/internal/observable/empty.ts` | `ParseError { span: Span { start: 2071, end: 2072 }, message: "expected `Colon`" ` |
| 20 | `other: ParseError { span: Span { start: 293, en` | 1 | `rxjs/src/internal/util/argsOrArgArray.ts` | `ParseError { span: Span { start: 293, end: 294 }, message: "expected `Colon`" }` |
| 21 | `other: ParseError { span: Span { start: 178, en` | 1 | `pino/test/types/pino-import.test-d.ts` | `ParseError { span: Span { start: 178, end: 179 }, message: "expected `from`" }` |
| 22 | `other: ParseError { span: Span { start: 279, en` | 1 | `ajv/lib/vocabularies/jtd/optionalProperties.ts` | `ParseError { span: Span { start: 279, end: 280 }, message: "expected `RParen`" }` |
| 23 | `other: ParseError { span: Span { start: 2291, e` | 1 | `rxjs/src/internal/operators/mergeInternals.ts` | `ParseError { span: Span { start: 2291, end: 2292 }, message: "expected `Colon`" ` |
| 24 | `other: ParseError { span: Span { start: 192, en` | 1 | `rxjs/src/internal/observable/range.ts` | `ParseError { span: Span { start: 192, end: 193 }, message: "expected `LBrace`" }` |
| 25 | `other: ParseError { span: Span { start: 329, en` | 1 | `rxjs/src/internal/operators/every.ts` | `ParseError { span: Span { start: 329, end: 330 }, message: "expected `LBrace`" }` |
| 26 | `other: ParseError { span: Span { start: 265, en` | 1 | `ajv/lib/vocabularies/applicator/allOf.ts` | `ParseError { span: Span { start: 265, end: 266 }, message: "expected `RParen`" }` |
| 27 | `other: ParseError { span: Span { start: 2982, e` | 1 | `rxjs/src/internal/observable/timer.ts` | `ParseError { span: Span { start: 2982, end: 2983 }, message: "expected `LBrace`"` |
| 28 | `other: ParseError { span: Span { start: 1355, e` | 1 | `rxjs/src/internal/operators/joinAllInternals.ts` | `ParseError { span: Span { start: 1355, end: 1356 }, message: "expected `Colon`" ` |
| 29 | `other: ParseError { span: Span { start: 285, en` | 1 | `rxjs/src/internal/operators/findIndex.ts` | `ParseError { span: Span { start: 285, end: 286 }, message: "expected `LBrace`" }` |
| 30 | `other: ParseError { span: Span { start: 1123, e` | 1 | `ajv/lib/2020.ts` | `ParseError { span: Span { start: 1123, end: 1124 }, message: "expected `Colon`" ` |

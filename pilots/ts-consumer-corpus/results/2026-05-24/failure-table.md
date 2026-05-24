# TCC failure-frequency table — 2026-05-24

Total files measured: 374. OK: 176 (47.1% parse-success).

Rows ranked by file count. Each row's `structural tag` names a TS feature / shape concept; an example file is given so the failure can be inspected. Sub-locale priority order is set by this table.

| Rank | Structural tag | Files | Example | Sample message |
|---:|---|---:|---|---|
| 1 | `method-return-annotation` | 46 | `ajv/lib/compile/errors.ts` | `ParseError { span: Span { start: 1921, end: 1922 }, message: "expected `RParen`"` |
| 2 | `generic-call` | 37 | `rxjs/src/internal/ajax/ajax.ts` | `ParseError { span: Span { start: 9085, end: 9086 }, message: "unexpected token i` |
| 3 | `uncategorized-unexpected-token` | 21 | `ajv/lib/compile/validate/boolSchema.ts` | `ParseError { span: Span { start: 1072, end: 1073 }, message: "unexpected token i` |
| 4 | `access-modifier` | 10 | `rxjs/src/internal/Subject.ts` | `ParseError { span: Span { start: 778, end: 779 }, message: "expected `LBrace`" }` |
| 5 | `readonly-modifier` | 10 | `ajv/lib/compile/codegen/code.ts` | `ParseError { span: Span { start: 375, end: 376 }, message: "expected class membe` |
| 6 | `decorator` | 9 | `rxjs/src/internal/AsyncSubject.ts` | `ParseError { span: Span { start: 298, end: 299 }, message: "expected `LBrace`" }` |
| 7 | `import-export-type` | 5 | `ajv/lib/compile/jtd/parse.ts` | `ParseError { span: Span { start: 12, end: 15 }, message: "expected `from`" }` |
| 8 | `lex-invalid-identifier` | 5 | `ajv/lib/compile/validate/index.ts` | `strip: ts-strip @18131: lex: LexError { kind: InvalidIdentifier, span: Span { st` |
| 9 | `keyof-type` | 2 | `rxjs/src/internal/operators/distinctUntilKeyChanged.ts` | `ParseError { span: Span { start: 239, end: 240 }, message: "expected `LBrace`" }` |
| 10 | `template-literal-type` | 2 | `ajv/lib/compile/ref_error.ts` | `ParseError { span: Span { start: 303, end: 304 }, message: "expected class membe` |
| 11 | `other: strip: ts-strip @28524: lex: LexError { ` | 1 | `ajv/lib/core.ts` | `strip: ts-strip @28524: lex: LexError { kind: InvalidNumeric, span: Span { start` |
| 12 | `other: ParseError { span: Span { start: 422, en` | 1 | `rxjs/src/internal/operators/expand.ts` | `ParseError { span: Span { start: 422, end: 423 }, message: "expected `LBrace`" }` |
| 13 | `other: ParseError { span: Span { start: 2790, e` | 1 | `rxjs/src/internal/ajax/errors.ts` | `ParseError { span: Span { start: 2790, end: 2791 }, message: "expected `RParen`"` |
| 14 | `other: ParseError { span: Span { start: 458, en` | 1 | `ajv/lib/vocabularies/applicator/items.ts` | `ParseError { span: Span { start: 458, end: 459 }, message: "expected `RParen`" }` |
| 15 | `other: ParseError { span: Span { start: 317, en` | 1 | `ajv/lib/vocabularies/validation/limitContains.ts` | `ParseError { span: Span { start: 317, end: 318 }, message: "expected `RParen`" }` |
| 16 | `other: ParseError { span: Span { start: 2535, e` | 1 | `rxjs/src/internal/observable/iif.ts` | `ParseError { span: Span { start: 2535, end: 2536 }, message: "expected `Colon`" ` |
| 17 | `other: ParseError { span: Span { start: 973, en` | 1 | `rxjs/src/internal/ajax/getXHRResponse.ts` | `ParseError { span: Span { start: 973, end: 974 }, message: "expected `Colon`" }` |
| 18 | `other: ParseError { span: Span { start: 279, en` | 1 | `ajv/lib/vocabularies/jtd/optionalProperties.ts` | `ParseError { span: Span { start: 279, end: 280 }, message: "expected `RParen`" }` |
| 19 | `other: ParseError { span: Span { start: 421, en` | 1 | `rxjs/src/internal/operators/mergeMapTo.ts` | `ParseError { span: Span { start: 421, end: 422 }, message: "expected `LBrace`" }` |
| 20 | `other: ParseError { span: Span { start: 293, en` | 1 | `rxjs/src/internal/util/argsOrArgArray.ts` | `ParseError { span: Span { start: 293, end: 294 }, message: "expected `Colon`" }` |
| 21 | `other: ParseError { span: Span { start: 629, en` | 1 | `ajv/lib/runtime/ucs2length.ts` | `ParseError { span: Span { start: 629, end: 629 }, message: "expected `RBrace`" }` |
| 22 | `other: ParseError { span: Span { start: 4985, e` | 1 | `rxjs/src/internal/observable/dom/WebSocketSubject.ts` | `ParseError { span: Span { start: 4985, end: 4986 }, message: "expected `RParen`"` |
| 23 | `other: ParseError { span: Span { start: 702, en` | 1 | `ajv/lib/vocabularies/validation/limitLength.ts` | `ParseError { span: Span { start: 702, end: 703 }, message: "expected `RParen`" }` |
| 24 | `other: ParseError { span: Span { start: 672, en` | 1 | `rxjs/src/internal/scheduler/intervalProvider.ts` | `ParseError { span: Span { start: 672, end: 673 }, message: "expected `RParen`" }` |
| 25 | `other: ParseError { span: Span { start: 1373, e` | 1 | `rxjs/src/internal/operators/max.ts` | `ParseError { span: Span { start: 1373, end: 1374 }, message: "expected `Colon`" ` |
| 26 | `other: ParseError { span: Span { start: 240, en` | 1 | `ajv/lib/vocabularies/jtd/metadata.ts` | `ParseError { span: Span { start: 240, end: 241 }, message: "expected `RParen`" }` |
| 27 | `other: ParseError { span: Span { start: 2982, e` | 1 | `rxjs/src/internal/observable/timer.ts` | `ParseError { span: Span { start: 2982, end: 2983 }, message: "expected `LBrace`"` |
| 28 | `other: ParseError { span: Span { start: 1263, e` | 1 | `ajv/lib/compile/validate/dataType.ts` | `ParseError { span: Span { start: 1263, end: 1264 }, message: "expected `Colon`" ` |
| 29 | `other: ParseError { span: Span { start: 400, en` | 1 | `ajv/lib/vocabularies/applicator/properties.ts` | `ParseError { span: Span { start: 400, end: 401 }, message: "expected `RParen`" }` |
| 30 | `other: ParseError { span: Span { start: 4176, e` | 1 | `rxjs/src/internal/operators/repeat.ts` | `ParseError { span: Span { start: 4176, end: 4177 }, message: "expected `Colon`" ` |

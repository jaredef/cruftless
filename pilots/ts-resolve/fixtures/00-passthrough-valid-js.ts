// TSR-EXT 2 smoke fixture: pure JavaScript inside a .ts extension.
// At this round, TsParser delegates to rusty-js-parser and erase is
// identity, so this must round-trip identically to the .js sibling.
let x = 1;
let y = 2;
console.log(x + y);

// TSR-EXT 4 Pred-tsr.4 twin: equivalent erased JavaScript of
// 01-end-to-end-hello.ts. Must produce byte-identical stdout under
// cruft.
function greet(g) {
    return "hello, " + g.who;
}

const target = { who: "world" };
console.log(greet(target));

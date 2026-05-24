// TSR-EXT 4 end-to-end smoke fixture: real .ts run through cruft.
interface Greeting { who: string; }

function greet(g: Greeting): string {
    return "hello, " + g.who;
}

const target: Greeting = { who: "world" };
console.log(greet(target));

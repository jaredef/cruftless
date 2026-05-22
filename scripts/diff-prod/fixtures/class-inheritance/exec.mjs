// F-category: ES2015 classes.

function canon(v) {
  return JSON.stringify(v, (_k, v) => {
    if (v && typeof v === "object" && !Array.isArray(v)) {
      const out = {}; for (const k of Object.keys(v).sort()) out[k] = v[k]; return out;
    }
    return v;
  });
}

const result = {};

// Basic class.
class Animal {
  constructor(name) { this.name = name; }
  speak() { return `${this.name} makes a sound`; }
}
{
  const a = new Animal("Cat");
  result.basic = {
    name: a.name,
    speak: a.speak(),
    instanceof_Animal: a instanceof Animal,
    instanceof_Object: a instanceof Object,
  };
}

// Inheritance.
class Dog extends Animal {
  constructor(name, breed) { super(name); this.breed = breed; }
  bark() { return `${this.name} barks (${this.breed})`; }
  speak() { return super.speak() + " (woof)"; }
}
{
  const d = new Dog("Rex", "Lab");
  result.extends = {
    name: d.name,
    breed: d.breed,
    bark: d.bark(),
    speak: d.speak(),
    inst_Dog: d instanceof Dog,
    inst_Animal: d instanceof Animal,
    inst_Object: d instanceof Object,
  };
}

// Static methods.
class Counter {
  static count = 0;
  static inc() { Counter.count++; return Counter.count; }
}
result.statics = {
  initial: Counter.count,
  after_1: Counter.inc(),
  after_2: Counter.inc(),
  after_3: Counter.inc(),
  final: Counter.count,
};

// Getters / setters.
class Temperature {
  constructor(c) { this._c = c; }
  get celsius() { return this._c; }
  set celsius(v) { this._c = v; }
  get fahrenheit() { return this._c * 9/5 + 32; }
}
{
  const t = new Temperature(20);
  const f1 = t.fahrenheit;
  t.celsius = 100;
  const f2 = t.fahrenheit;
  result.accessors = { f1, f2 };
}

// Class fields (public).
class Point {
  x = 0;
  y = 0;
  constructor(x, y) { this.x = x; this.y = y; }
  manhattan() { return Math.abs(this.x) + Math.abs(this.y); }
}
{
  const p = new Point(3, -4);
  result.fields = { x: p.x, y: p.y, m: p.manhattan() };
}

// Method chaining via this-return.
class Builder {
  constructor() { this.parts = []; }
  add(p) { this.parts.push(p); return this; }
  build() { return this.parts.join("-"); }
}
result.chain = new Builder().add("a").add("b").add("c").build();

// Symbol.hasInstance custom.
class EvenNumbers {
  static [Symbol.hasInstance](x) { return typeof x === "number" && x % 2 === 0; }
}
result.has_instance = { even_4: 4 instanceof EvenNumbers, odd_5: 5 instanceof EvenNumbers };

console.log(canon(result));

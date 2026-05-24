// TSR-EXT 4: generics on function head + union + array type.
interface Item { name: string; price: number; }

function total<T extends Item>(items: T[]): number {
    let sum: number = 0;
    for (const it of items) {
        sum = sum + it.price;
    }
    return sum;
}

const cart: Item[] = [
    { name: "apple", price: 1 },
    { name: "bread", price: 3 },
    { name: "milk", price: 2 },
];
console.log(total(cart));

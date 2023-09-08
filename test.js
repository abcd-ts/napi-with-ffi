const { getJsonCst } = require(".");

const json = { name: "Alice", like: "Rust" };
console.log(getJsonCst(JSON.stringify(json)));

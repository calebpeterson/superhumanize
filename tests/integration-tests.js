// If this fails, you probably need to run `wasm-pack build --target nodejs`
const { superhumanize } = require("../pkg/superhumanize");

const assert = (got, expected) => {
  if (got !== expected) {
    throw new Error(`🛑 Assertion failed. Got ${got} and expected ${expected}`);
  }
};

assert(superhumanize("tomSmith"), "Tom Smith");
assert(superhumanize("tom-smith"), "Tom Smith");

console.log("✅ All tests passed");

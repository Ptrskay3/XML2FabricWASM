// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('./pkg');

rust.then((m) => ((global.parse = m.parse), (global.init_panic_hook = m.init_panic_hook))).catch(console.error);

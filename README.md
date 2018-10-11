# TODO MVC using wasm-bingen and web-sys

[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) and [web-sys](https://rustwasm.github.io/wasm-bindgen/api/web_sys/) coded [TODO MVC](https://todomvc.com/)

The code was rewritten from the [ECMA 6 version](http://todomvc.com/examples/vanilla-es6/).

The core differences are:
- Having an [Element wrapper](/src/element.rs) that takes care of dyn refs in web-sys
- A [Scheduler](/src/scheduler.rs) that allows Controller and View to speak to each other by emulating something similar to the JS event loop.

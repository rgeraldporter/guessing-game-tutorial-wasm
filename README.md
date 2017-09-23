# Rust "Guessing Game" Tutorial, Web Assembly Edition

This is a web assembly variant of of the Rust "Guessing Game Tutorial". There's no practical use for this codebase, other than to demonstrate some very simple communication between Javascript and a compiled Rust binary.

# How to demo

Very simply, you can just clone and then run your favourite web server tool on `/site/` directory; for example, [Node.js's `http-server`](https://www.npmjs.com/package/http-server). If you're making code alterations you'll need to make sure to install Rust as well as `rustup` and `emscripten`; [details can be found here](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627).

Once you're all set up, you can just run `make` to do the compile and copying of the new js/wasm files into the site directory.

# License

The MIT License (MIT)

Copyright (c) 2017 Robert Gerald Porter

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

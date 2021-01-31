# macro_io

Macro_io provides procedural macros to do IO during compile time in the Rust programming language.
This could be useful when you need to load string literals from an external file at compile time.
This way the contents gets processed and (type) checked at compile time.
An example would be the format string of the fmt! macro.

At the time of writing only one macro is available: `read_file!()`. The macro is used as follows:

```
hello_world.txt: Hello world!
fmt.txt:         {}: {}
```

```rust
use macro_io::read_file;

let hello = read_file!("hello_world.txt");
assert_eq!(hello, "Hello world!");
let formatted = fmt!(read_file!("fmt.txt"), "foo", "bar");
assert_eq!(formatted, "foo: bar");
```

## Contributing
All contributions are welcome, in the form of issues or pull requests.
I would like to extent the crate with more functionality, so let me know what would be useful to you and what you would like to see.

The functionality of the crate is extensively tested and this is also expected for contributions.
Both passing tests as well as failing compile tests are present.

## License (MIT)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

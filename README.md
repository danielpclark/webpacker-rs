# Webpacker-rs

Webpacker-rs is a Rust wrapper for using WebpackerCli/Webpacker/Webpack in your Rust web
framework's deployment and asset management.

## Usage

Add the following to your Cargo.toml

```toml
[dependencies]
webpacker = "~0.1"

[build-dependencies]
webpacker = "~0.1"
```

In your build script you can do the following:

```rust
use webpacker;
fn main() {
    // Validating dependencies…
    assert!(webpacker::valid_project_dir());

    // Comiling assets…
    let _ = webpacker::compile();
}
```

And then in your application during web start up you can generate
a hash of the file manifest with:

```rust
use webpacker;

// Returns webpacker::Manifest object
// which has a `get` method to look up
// each file.
webpacker::manifest()
```

You can use the `Manifest` object in your routing tables.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/danielpclark/webpacker-cli


## License

The gem is available as open source under the terms of the [GNU Lesser General Public License version 3](https://opensource.org/licenses/LGPL-3.0).

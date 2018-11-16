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

// Returns `Manifest` object which is an
// immutable Hashmap
webpacker::manifest()
```

You can use the `Manifest` object in your routing tables.

## Gotham

In [Gotham](https://gotham.rs/) one way you can use the manifest for the router as follows:

```rust
pub fn router() -> Router {
    build_simple_router(|route| {
        for (key, value) in webpacker::manifest(None).unwrap() {
            route
                .get(&format!("public/{}", key))
                .to_file(format!("public{}", value));
        }
    })
}
```

And in each of your webpages you link to your assets as though they were in the `public/` folder. 
This will map the normal file names like `application.js` to their hashed version
`application-285f2db5acb1800187f0.js`.  _I'm not sure having the router do this lets the cache
invalidation work as intended._

The recommended way to use this is to have a helper method write the mapped file name right to
the generated webpage HTML source.  So if you're using [handlebars](https://github.com/sunng87/handlebars-rust)
or [tera](https://github.com/Keats/tera) then you could do something like:

    <script src="public{{ manifest.get("application.js") }}"></script>

Note the manifest value will have a preceeding slash so you don't need one after the folder name `public`.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/danielpclark/webpacker-cli


## License

The gem is available as open source under the terms of the [GNU Lesser General Public License version 3](https://opensource.org/licenses/LGPL-3.0).

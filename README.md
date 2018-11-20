# Webpacker-rs

Webpacker-rs is a Rust wrapper for using WebpackerCli/Webpacker/Webpack in your Rust web
framework's deployment and asset management.

## Usage

Add the following to your Cargo.toml

```toml
[dependencies]
webpacker = "~0.3"

[build-dependencies]
webpacker = "~0.3"
```

Before you can build you need to initialize the webpacker environment with:

    gem install webpacker_cli
    webpacker-cli init

Now your build script will be able to compile. In your build script you can do the following:

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
`/packs/application-285f2db5acb1800187f0.js`.  _I'm not sure having the router do this lets the cache
invalidation work as intended._

The recommended way to use this is to have a helper method write the mapped file name right to
the generated webpage HTML source.  So if you're using [tera](https://github.com/Keats/tera) then you
could do something like:

```rust
pub static ASSET_DIRECTORY: &'static str = "public";

lazy_static! {
    pub static ref MANIFEST: Manifest = webpacker::manifest(None).unwrap();
}

mod assets {
    use super::{ASSET_DIRECTORY, MANIFEST};
    use webpacker::asset_path::AssetPath;
    use std::ops::Deref;

    pub fn source(key: &str) -> String {
        AssetPath::new(ASSET_DIRECTORY, key, MANIFEST.deref()).into()
    }
}

pub fn index_page(state: State) -> (State, (mime::Mime, String)) {
    let mut context = Context::new();
    context.insert("application_source", &assets::source("application.js"));

    let rendered = TERA.render("landing_page/index.html.tera", &context).unwrap();

    (state, (mime::TEXT_HTML, rendered))
}
```

```html
<script src="{{ application_source }}"></script>
```

Doing this preferred way means you should have the folder `/public/packs/*` routed with something like this:

```rust
pub fn router() -> Router {
    build_simple_router(|route| {
        route.get("public/packs/*").to_dir(
            FileOptions::new("public/packs")
                .with_cache_control("no-cache")
                .with_gzip(true)
                .build(),
        );
    })  
}
```

`FileOptions` here provides Async file support.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/danielpclark/webpacker-cli


## License

The gem is available as open source under the terms of the [GNU Lesser General Public License version 3](https://opensource.org/licenses/LGPL-3.0).

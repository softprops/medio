# medio

[![Build Status](https://travis-ci.org/softprops/medio.svg?branch=master)](https://travis-ci.org/softprops/medio)

> rustlang bindings to [Medium.com](https://medium.com/) api

## apidocs

Find them [here](https://softprops.github.io/medio)

## install

add the following to your Cargo.toml file

```toml
[depenencies]
medio = "0.1.0"
```

## usage

To get started, you'll want grab an integration token from you're medium.com accounts [settings](https://medium.com/me/settings)
page.

```rust
extern crate hyper;
extern crate medio;

use hyper::Client;
use medio::{Medium, NewPost};
use std::default::Default;

fn main() {
    let client = Client::new();
    let medium = Medium::new(
        &client,
        Some("t0k3n")
    );
    // fetch a reference to authenticated user info
    let me = medium.me().unwrap();
    medium.users(&me.id).post(
        &NewPost {
           title: "hello rust",
           content: &format(
              r#" # hello medium.

                    meet rust.

                    love @{}.
                "#, me.username
           ),
           tags: Some(
               vec!["rust"]
           ),
           ..Default::default()
        }
    );
}

```

### resources

* medium [api docs](https://github.com/Medium/medium-api-docs)

Doug Tangren (softprops) 2015

extern crate hyper;
extern crate rustc_serialize;

mod rep;


use hyper::Client;
use hyper::client::{IntoUrl, RequestBuilder};
use hyper::header::Authorization;
use rustc_serialize::json;
pub use rep::*;
use std::io::{Read, Result};

/// reference point for creating new posts
pub struct UserRef<'a> {
  medium: &'a Medium<'a>,
  id: &'a str
}

impl<'a> UserRef<'a> {
  pub fn new(medium: &'a Medium<'a>, id: &'a str) -> UserRef<'a> {
    UserRef {
      medium: medium,
      id: id
    }
  }

  /// posts a new story
  pub fn post(&self, post: &Post) -> String {
    String::new()
  }
}

/// Entry point for access to medium.com api
pub struct Medium<'a> {
  client: &'a Client,
  token: Option<&'a str>,
  host: &'a str
}

impl<'a> Medium <'a> {
  /// creates a new instance of a Medium client
  /// token can be acquired [here](https://medium.com/me/settings)
  pub fn new(
    client: &'a Client,
    token: Option<&'a str>
  ) -> Medium<'a> {
    Self::host(
      client, token, "https://api.medium.com"
    )
  }

  /// creates an instance of a Medium client for a specific host
  pub fn host(
    client: &'a Client,
    token: Option<&'a str>,
    host: &'a str
  ) -> Medium<'a> {
    Medium {
      client: client,
      token: token,
      host: host
    }
  }

  /// access to the authentied members user info
  pub fn me(&self) -> Result<User> {
    let body = try!(self.get("/v1/me"));
    let data: Data<User> = json::decode(&body).unwrap();
    Ok(data.data)
  }

  /// access to a user reference
  pub fn user(&self, id: &'a str) -> UserRef {
    UserRef::new(self, id)
  }

  fn request<U: IntoUrl>(
    &self, request_builder: RequestBuilder<'a, U>
      ) -> Result<String> {
    let authenticated = match self.token {
      Some(token) =>
        request_builder.header(
          Authorization(format!("Bearer {}", token))
        ),
        _ => request_builder
    };
    let mut res = authenticated.send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Ok(body)
  }

  fn get(&self, uri: &str) -> Result<String> {
    let url = format!("{}{}", self.host, uri);
    self.request(
      self.client.get(&url)
    )
  }
}

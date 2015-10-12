extern crate hyper;

use hyper::Client;

pub enum ContentFormat {
  html, markdown
}

pub enum PublishStatus {
  public, draft, unlisted
}

pub enum License {
  cc40
}

pub struct Post<'a> {
  title: &'a str,
  contentFormat: ContentFormat,
  content: &'a str,
  tags: Option<Vec<&'a str>>,
  canonicalUrl: Option<&'a str>,
  publishStatus: Option<PublishStatus>,
  license: Option<License>
}

struct User<'a> {
  medium: &'a Medium<'a>,
  id: &'a str
}

impl<'a> User<'a> {
  pub fn new(medium: &'a Medium<'a>, id: &'a str) -> User<'a> {
    User {
      medium: medium,
      id: id
    }
  }

  pub fn post(&self, post: &Post) -> String {
    String::new()
  }
}

struct Medium<'a> {
  client: &'a Client,
  token: Option<&'a str>,
  host: &'a str
}

impl<'a> Medium <'a> {
  pub fn new(
    client: &'a Client,
    token: Option<&'a str>
  ) -> Medium<'a> {
    Self::host(client, token, "https://api.medium.com")
  }

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

  pub fn me(&self) -> String {
    String::new()
  }

  pub fn user(&self, id: &'a str) -> User {
    User::new(self, id)
  }
}

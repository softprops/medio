extern crate hyper;
extern crate rustc_serialize;

mod rep;

use hyper::Client;
use hyper::method::Method;
use hyper::header::ContentType;
use hyper::header::Authorization;
use rustc_serialize::{Decodable, json};
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
    pub fn post(&self, post: &NewPost) -> Result<Post> {
        let data = json::encode(&post).unwrap();
        self.medium.post::<Data<Post>>(
            &format!("/v1/users/{}/posts", self.id),
            data.as_bytes()
        ).map(|d| d.data)
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
        self.get::<Data<User>>("/v1/me").map(|d| d.data)
    }

    /// access to a user reference
    pub fn users(&self, id: &'a str) -> UserRef {
        UserRef::new(self, id)
    }

    fn request<T>(
        &self,
        method: Method,
        uri: &str,
        body: Option<(ContentType, &'a [u8])>
     ) -> Result<T> where T: Decodable {
        let url = format!("{}{}", self.host, uri);
        let request_builder = self.client.request(
            method,
            &url
        );
        let authenticated = match self.token {
            Some(token) =>
                request_builder.header(
                    Authorization(format!("Bearer {}", token))
                ),
            _ => request_builder
        };
        let mut res = match body {
            Some((typ, body)) => {
                authenticated.header(
                    typ
                    )
                    .body(body)
                    .send()
                    .unwrap()
            }, _ => authenticated.send().unwrap()
        };
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        Ok(json::decode::<T>(&body).unwrap())
    }

    fn post<T>(
        &self, uri: &str, message: &[u8]
    ) -> Result<T> where T: Decodable {
        self.request(
            Method::Post,
            uri,
            Some((ContentType::json(), message))
        )
    }

    fn get<T>(
        &self, uri: &str
    ) -> Result<T> where T: Decodable {
        self.request(
            Method::Get,
            uri,
            None
        )
    }
}

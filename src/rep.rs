use rustc_serialize::{Decodable, Encodable, Encoder};
use std::default::Default;

#[derive(Debug, RustcDecodable)]
pub struct User {
  pub id: String,
  pub username: String,
  pub url: String,
  pub imageUrl: String
}

#[derive(Debug, RustcDecodable)]
pub struct Data<D: Decodable> {
  pub data: D
}

#[derive(Debug, RustcDecodable)]
pub enum ContentFormat {
  Html, Markdown
}

impl Encodable for ContentFormat {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match *self {
            ContentFormat::Html => encoder.emit_str("html"),
            ContentFormat::Markdown => encoder.emit_str("markdown"),
        }
    }
}

impl Default for ContentFormat {
  fn default() -> ContentFormat {
    ContentFormat::Markdown
  }
}

#[derive(Debug, RustcDecodable)]
pub enum PublishStatus {
    Public, Draft, Unlisted
}

impl Default for PublishStatus {
  fn default() -> PublishStatus {
    PublishStatus::Public
  }
}

impl Encodable for PublishStatus {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match *self {
            PublishStatus::Public => encoder.emit_str("html"),
            PublishStatus::Draft => encoder.emit_str("markdown"),
            PublishStatus::Unlisted => encoder.emit_str("unlisted")
        }
    }
}

#[derive(Debug, RustcDecodable)]
pub enum License {
  Cc40By,
  Cc40BySa,
  Cc40ByNd,
  Cc40ByNc,
  Cc40ByNcNd,
  Cc40ByNcSa,
  Cc40Zero,
  PublicDomain,
  AllRightsReserved
}

impl Encodable for License {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match *self {
            License::Cc40By => encoder.emit_str("cc-40-by"),
            License::Cc40BySa => encoder.emit_str("cc-40-by-sa"),
            License::Cc40ByNd => encoder.emit_str("cc-40-by-nd"),
            License::Cc40ByNc => encoder.emit_str("cc-40-by-nc"),
            License::Cc40ByNcNd => encoder.emit_str("cc-40-by-nc-nd"),
            License::Cc40ByNcSa => encoder.emit_str("cc-40-by-nc-sa"),
            License::Cc40Zero => encoder.emit_str("cc-40-zero"),
            License::PublicDomain => encoder.emit_str("public-domain"),
            License::AllRightsReserved => encoder.emit_str("all-rights-reserved")
        }
    }
}

impl Default for License {
  fn default() -> License {
    License::AllRightsReserved
  }
}

#[derive(Debug, RustcEncodable, Default)]
pub struct NewPost<'a> {
  pub title: &'a str,
  pub content_format: ContentFormat,
  pub content: &'a str,
  pub tags: Option<Vec<&'a str>>,
  pub canonicalUrl: Option<&'a str>,
  pub publishStatus: Option<PublishStatus>,
  pub license: Option<License>
}

#[derive(Debug, RustcDecodable)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub authorId: String,
    pub tags: Vec<String>,
    pub url: String,
    pub publishStatus: PublishStatus,
    pub publishedAt: u64,
    pub license: String
}

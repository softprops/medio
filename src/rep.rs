use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use std::default::Default;


/// Represents a Medium user
#[derive(Debug, RustcDecodable)]
pub struct User {
  pub id: String,
  pub username: String,
  pub url: String,
  pub imageUrl: String
}

/// An envelope for data that Medium response with
#[derive(Debug, RustcDecodable)]
pub struct Data<D: Decodable> {
  pub data: D
}

/// Indicator for type of content
#[derive(Debug)]
pub enum ContentFormat {
  Html, Markdown
}

impl Decodable for ContentFormat {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<ContentFormat, D::Error> {
        decoder.read_enum("ContentFormat", move |this| {
            this.read_enum_variant(&["html", "markdown"], move |this, idx| {
                match idx {
                    0 => Ok(ContentFormat::Html),
                    1 => Ok(ContentFormat::Html),
                    _ => unreachable!()
                }
            })
        })
    }
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

/// One of three statuses a hosted post may have
#[derive(Debug)]
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
            PublishStatus::Public => encoder.emit_str("public"),
            PublishStatus::Draft => encoder.emit_str("draft"),
            PublishStatus::Unlisted => encoder.emit_str("unlisted")
        }
    }
}

impl Decodable for PublishStatus {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<PublishStatus, D::Error> {
        decoder.read_enum("PublishStatus", move |this| {
            this.read_enum_variant(&["public", "draft", "unlisted"], move |this, idx| {
                match idx {
                    0 => Ok(PublishStatus::Public),
                    1 => Ok(PublishStatus::Draft),
                    2 => Ok(PublishStatus::Unlisted),
                    _ => unreachable!()
                }
            })
        })
    }
}

/// Licences that may be associated with posts
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

/// Payload for creating new posts
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

/// Represents a saved post
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

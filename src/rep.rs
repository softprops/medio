use rustc_serialize::Decodable;
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

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum ContentFormat {
  html, markdown
}

impl Default for ContentFormat {
  fn default() -> ContentFormat {
    ContentFormat::markdown
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum PublishStatus {
  public, draft, unlisted
}

impl Default for PublishStatus {
  fn default() -> PublishStatus {
    PublishStatus::public
  }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum License {
  cc40by,
  cc40bySa,
  cc40byNd,
  cc40byNc,
  cc40byNcNd,
  cc40byNcSa,
  cc40zero,
  publicDomain,
  allRightsReserved
}

impl Default for License {
  fn default() -> License {
    License::allRightsReserved
  }
}

#[derive(Debug, RustcEncodable)]
pub struct Post<'a> {
  title: &'a str,
//  #[serde(rename(json="contentFormat"))]
  content_format: ContentFormat,
  content: &'a str,
  tags: Option<Vec<&'a str>>,
 // #[serde(rename(json="canonicalUrl"))]
  canonicalUrl: Option<&'a str>,
 // #[serde(rename(json="publishStatus"))]
  publishStatus: Option<PublishStatus>,
  license: Option<License>
}

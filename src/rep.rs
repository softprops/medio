use rustc_serialize::Decodable;

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

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum PublishStatus {
  public, draft, unlisted
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum License {
  cc40
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

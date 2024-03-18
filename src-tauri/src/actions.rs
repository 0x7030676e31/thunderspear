use crate::reader::{SLICE_SIZE, CLUSTER_CAP, Cluster};

use reqwest::{Client, StatusCode};
use futures::{stream, future};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UploadDetail {
  pub upload_url: String,
  pub upload_filename: String,
}

#[derive(Deserialize)]
struct UploadDetails {
  attachments: Vec<UploadDetail>,
}

#[derive(Deserialize)]
struct RateLimit {
  retry_after: f32,
}

#[derive(Deserialize)]
pub struct MessageAttachment {
  pub url: String,
}

#[derive(Deserialize)]
pub struct Message {
  pub id: String,
  pub attachments: Vec<MessageAttachment>,
}


pub async fn preupload(auth: &str, channel: &str, idx: usize, size: usize) -> Result<Vec<UploadDetail>, f32> {
  let slices = (size + SLICE_SIZE - 1) / SLICE_SIZE;
  let slices = (0..slices).map(|i| {
    let size = std::cmp::min(size - i * SLICE_SIZE, SLICE_SIZE);
    let name = idx * CLUSTER_CAP + i;
  
    format!(r#"{{"file_size":{},"filename":"{}","id":"0","is_clip":false}}"#, size, name)
  });

  let body = format!(r#"{{"files":[{}]}}"#, slices.collect::<Vec<_>>().join(","));
  
  let client = Client::new();
  let req = client.post(format!("https://discord.com/api/v9/channels/{}/attachments", channel))
    .header("Content-Type", "application/json")
    .header("Authorization", auth)
    .body(body)
    .send()
    .await
    .unwrap();

  match req.status() {
    StatusCode::TOO_MANY_REQUESTS => {
      let rate_limit: RateLimit = req.json().await.unwrap();
      Err(rate_limit.retry_after)
    }
    StatusCode::OK => {
      let details: UploadDetails = req.json().await.unwrap();
      Ok(details.attachments)
    }
    _ => {
      panic!("Unexpected status code: {}", req.status());
    }
  }
}

pub async fn upload(auth: &str, details: &Vec<UploadDetail>, mut cluster: Cluster) {
  let client = Client::new();
  
  let futures = details.iter().map(|detail| {
    let stream = stream::iter(cluster.next_slice().unwrap());
    client.put(&detail.upload_url)
      .header("Content-Type", "application/octet-stream")
      .header("Authorization", auth)
      .body(reqwest::Body::wrap_stream(stream))
      .send()
  });

  future::join_all(futures).await;
}

pub async fn send_message(auth: &str, channel: &str, attachments: &[UploadDetail], idx: usize) -> Result<String, f32> {
  let client = Client::new();
  
  let offset = idx * CLUSTER_CAP;
  let attachments = attachments.iter().enumerate().map(|(i, attachment)| {
    let name = offset + i;
    format!(r#"{{"filename":"{}","uploaded_filename":"{}","id":"{}"}}"#, name, attachment.upload_filename, i)
  });

  let body = format!(r#"{{"attachments":[{}],"channel_id":"{}","content":"","type":0,"sticker_ids":[]}}"#, attachments.collect::<Vec<_>>().join(","), channel);
  
  let req = client.post(format!("https://discord.com/api/v9/channels/{}/messages", channel))
    .header("Content-Type", "application/json")
    .header("Authorization", auth)
    .body(body)
    .send()
    .await
    .unwrap();

  match req.status() {
    StatusCode::TOO_MANY_REQUESTS => {
      let rate_limit: RateLimit = req.json().await.unwrap();
      Err(rate_limit.retry_after)
    }
    StatusCode::OK => {
      let message: Message = req.json().await.unwrap();
      Ok(message.id)
    }
    _ => {
      panic!("Unexpected status code: {}", req.status());
    }
  }
}
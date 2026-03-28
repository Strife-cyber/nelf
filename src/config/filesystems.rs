use aws_sdk_s3 as s3;
use aws_config::BehaviorVersion;

pub async fn connect() -> Result<s3::Client, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let endpoint_url = std::env::var("AWS_URL")
        .expect("AWS_URL must be set");

    let creds = s3::config::Credentials::new(
        std::env::var("AWS_ACCESS_KEY_ID")?,
        std::env::var("AWS_SECRET_ACCESS_KEY")?,
        None,
        None,
        "manual"
    );

    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(endpoint_url)
        .credentials_provider(creds)
        .load()
        .await;

    let s3_config = s3::config::Builder::from(&config)
        .force_path_style(true)
        .build();

    let client = s3::Client::from_conf(s3_config);

    Ok(client)
}

pub async fn upload(
    client: &s3::Client,
    key: &str,
    content: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let bucket = std::env::var("AWS_BUCKET")?;

    ensure_bucket(client, &bucket).await;

    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(s3::primitives::ByteStream::from(content))
        .send()
        .await?;

    Ok(())
}

pub async fn download(
    client: &s3::Client,
    key: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let bucket = std::env::var("AWS_BUCKET")?;

    ensure_bucket(client, &bucket).await;

    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    let data = resp.body.collect().await?;

    Ok(data.into_bytes().to_vec())
}

pub async fn ensure_bucket(client: &s3::Client, bucket: &str) {
    let _ = client.create_bucket().bucket(bucket).send().await;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let client = connect().await;

        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_s3_list_buckets() {
        let client = connect().await.unwrap();

        let result = client.list_buckets().send().await;

        println!("{:?}", result);

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_upload_and_download() {
        let client = connect().await.unwrap();

        let key = "test.txt";
        let content = b"hello from rust".to_vec();

        print!("{:?}", client.config());

        // upload
        let up = upload(&client, key, content.clone()).await;

        if let Err(e) = up {
            panic!("Upload failed: {:?}", e);
        }

        assert!(up.is_ok());

        // download
        let down = download(&client, key).await.unwrap();

        assert_eq!(down, content);
    }
}

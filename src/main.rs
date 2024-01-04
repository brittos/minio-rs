use minio::s3::args::{BucketExistsArgs, MakeBucketArgs, UploadObjectArgs};
use minio::s3::client::Client;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    
    // setting up the url server s3
    let endpoint = env::var("SERVER_ENDPOINT").expect("SERVER_ENDPOINT is set and a valid String");
    let base_url = endpoint.parse::<BaseUrl>()?;

    // setting up the credentials
    let key = env::var("ACCESS_KEY").expect("ACCESS_KEY is set and a valid String");
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY is set and a valid String");

    let static_provider = StaticProvider::new(
        &key,
        &secret,
        None,
    );

    let client = Client::new(
        base_url.clone(),
        Some(Box::new(static_provider)),
        None,
        Some(true),  // Adiciona a opção ignore_cert_check

    )
    .unwrap();

    //let bucket_name = "minio-rs";
    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME is set and a valid String");

    // Check 'minio-rs' bucket exist or not.
    let exists = client
        .bucket_exists(&BucketExistsArgs::new(&bucket_name).unwrap())
        .await
        .unwrap();

    // Make 'minio-rs' bucket if not exist.
    if !exists {
        client
            .make_bucket(&MakeBucketArgs::new(&bucket_name).unwrap())
            .await
            .unwrap();
    }

    // Upload './test/projetos.tar.gz' as object name
    // 'projetos.tar.gz' to bucket 'minio-rs'.
    client
        .upload_object(
            &mut UploadObjectArgs::new(
                &bucket_name,
                "projetos.tar.gz",
                "./test/projetos.tar.gz",
            )
            .unwrap(),
        )
        .await
        .unwrap();

    println!("🚀'/test/projetos.tar.gz' is successfully uploaded as object 'projetos.tar.gz' to bucket 'minio-rs'.");
    Ok(())
}
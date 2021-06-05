use std::env;
use s3::bucket::Bucket;
use s3::credentials::Credentials;
use s3::region::Region;

pub struct S3Repository {
    bucket: Bucket
}

impl S3Repository {
    pub fn new() -> Self {
        let access_key = env::var("AWS_ACCESS_KEY_ID").ok();
        let secret_key = env::var("AWS_SECRET_ACCESS_KEY").ok();
        let credentials: Credentials = Credentials::new(access_key, secret_key, None, None);
        let region: Region = env::var("AWS_DEFAULT_REGION").unwrap().parse().unwrap();
        S3Repository {
            bucket: Bucket::new(env::var("BUCKET_NAME").unwrap().as_str(), region, credentials)
        }
    }

    pub fn upload(&self, name: String, data: &Vec<u8>, mime: String) -> Result<(), Box<dyn std::error::Error>> {
        let (_, _) = self.bucket.put(&name.as_str(), data, &mime.as_str())?;
        Ok(())
    }

    pub fn delete(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        let (_, _) = self.bucket.delete(&name.as_str())?;
        Ok(())
    }
}

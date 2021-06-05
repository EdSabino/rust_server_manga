use sha2::{Sha256, Digest};

pub trait HashPassword {
    fn password(&self) -> String;

    fn hash_password(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.password());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
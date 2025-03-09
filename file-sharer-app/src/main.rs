use elliptic_curve::sec1::ToEncodedPoint;
use elliptic_curve::{SecretKey, PublicKey};
use ipfs_api::IpfsClient;
use rocket::{get, post, routes, serde::json::Json};
use serde::{Deserialize, Serialize};
use simple_identity::{IdentityManager, UserIdentity};
use simple_token::{TokenManager, AccessToken};
use risc0_zkvm::{default_prover, ExecutorEnv};
use anyhow::Result;
use rand_core::OsRng;
use hex;

#[derive(Serialize, Deserialize)]
struct DocumentData {
    file_name: String,
    content_base64: String,
}

#[derive(Serialize, Deserialize)]
struct UploadResponse {
    ipfs_hash: String,
    public_key: String,
}

#[post("/upload", format = "json", data = "<document>")]
async fn upload(document: Json<DocumentData>) -> Json<UploadResponse> {
    // Generate an ECC Key pair
    let secret_key = SecretKey::random(&mut OsRng);
    let public_key = PublicKey::from_secret_scalar(secret_key.to_nonzero_scalar());

    // Encrypt data using recipient or internal business logic (Simulated)
    // Here you would use ECC-based encryption (hybrid with symmetric AES is recommended)
    let content = base64::decode(&document.content_base64).unwrap();

    // IPFS upload
    let client = IpfsClient::default();
    let data = content.to_vec();
    let ipfs_response = client.add(data.as_slice()).await.unwrap();

    Json(UploadResponse {
        ipfs_hash: ipfs_response.hash,
        public_key: hex::encode(public_key.to_encoded_point(false).as_bytes()),
    })
}

#[get("/get/<hash>")]
async fn fetch_document(hash: &str) -> Result<Vec<u8>> {
    let client = IpfsClient::default();
    let data = client.cat(hash).await?;
    Ok(data.collect::<Vec<u8>>().await)
}

pub fn create_user_identity(username: &str) -> UserIdentity {
    IdentityManager::new_user(username)
}

pub fn issue_access_token(identity: &UserIdentity) -> AccessToken {
    TokenManager::generate(identity.username.clone())
}

pub fn generate_identity_proof(identity: &UserIdentity) -> Result<Vec<u8>> {
    // Example proving identity username length >= 3 without revealing actual username
    let env = ExecutorEnv::builder()
        .write(&identity.username)?
        .build()?;

    let prover = default_prover();
    let receipt = prover.prove(env, "your_zk_risc0_method")?;

    Ok(receipt.journal.bytes)
}

pub fn verify_identity_proof(proof: &[u8]) -> bool {
    use risc0_zkvm::verify::verify_receipt;

    if let Ok(_) = verify_receipt(proof) {
        true
    } else {
        false
    }
}

// Rocket App Entry
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = rocket::build()
        .mount("/", routes![upload, fetch_document])
        .launch()
        .await?;

    Ok(())
}
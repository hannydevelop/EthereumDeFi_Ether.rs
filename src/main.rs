mod transfer;
mod sign;

#[tokio::main]
async fn main() {
    transfer::transfers().await.ok();
    sign::signs().await.ok();
}
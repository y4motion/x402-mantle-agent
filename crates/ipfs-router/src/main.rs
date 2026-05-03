mod config;
mod engine;
mod network;

#[tokio::main]
async fn main() {
    println!("[IPFS Router] GIGANTOMANIA Vector 7 Online.");
    
    network::run_router_loop().await;
}

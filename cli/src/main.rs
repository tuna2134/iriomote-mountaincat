use iriomote_core::IriomoteCore;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = if let Some(filename) = args.get(1) {
        filename
    } else {
        println!("ファイル名を指定してください。");
        return Ok(());
    };
    let iriomote = IriomoteCore::new()?;
    if let Some(result) = iriomote.predict(tokio::fs::read(filename).await?).await? {
        println!("イリオモテヤマネコの確率: {}", (1.0 - result) * 100.0);
    } else {
        println!("なんらかの問題で推論できませんでした。");
    }
    Ok(())
}

mod push;

use push::Push;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Push::from_args();
    push::run(
        env::var("DEVTO_API_KEY")
            .map_err(|_| anyhow::anyhow!(
                "Please export a DEVTO_API_KEY env variable.\n  ▶ You can generate one by visiting https://dev.to/settings/account"
            ))?,
        args,
    )
    .await?;
    Ok(())
}

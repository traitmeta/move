use sui_sdk::SuiClient;

pub struct ExplorerCli {
    cli: SuiClient,
}

impl ExplorerCli {
    async fn new(url: &str) -> Self {
        let sui = SuiClient::new_rpc_client(url, None).await.unwrap();
        Self { cli: sui }
    }

    async fn get_tx_version(&self) -> u64 {
        self.cli
            .read_api()
            .get_total_transaction_number()
            .await
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::block::ExplorerCli;

    #[test]
    fn test_get_total_tx_number() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let cli = rt.block_on(ExplorerCli::new("https://fullnode.devnet.sui.io:443"));
        let num = rt.block_on(cli.get_tx_version());
        println!("{}", num);
        assert_eq!(num > 50799, true, "split?");
    }
}

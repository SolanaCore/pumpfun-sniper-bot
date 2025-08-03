#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedisManager {
    redis_port: u16,
    redis_url: String,
    // Optional password for Redis authentication
    redis_password: Option<String>,
}

impl RedisManager {
    fn new(&mut self, redis_port: u16, redis_password: Option<String>) -> Self {
        let redis_url = format!("redis://{}:{}", redis_password.is_some(), redis_port);
        RedisManager {
            redis_port,
            redis_url,
            redis_password,
    }
    fn get_redis_url(&self) -> &str {
        &self.redis_url
    }
    fn subscribe(&self, channel_name: &str) -> Result<(), redis::RedisError> {
        let client = redis::Client::open(self.redis_url.clone())?;
        let mut pubsub_con = client.get_async_pubsub().await?;

            pubsub_con.subscribe("transaction_channel").await?;
        println!("Subscribed to 'transaction_channel' ");

        loop {
            let msg = pubsub_con.on_message().await?;
            let payload: String = msg.get_payload()?;
            println!("Received message: {}", payload);
            //Payload:
              // program_id
              // tx account
              // tx data    
            let transaction: Transaction = serde_json::from_str::<Transaction>(&payload).map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Failed to deserialize payload", e)))?;
            println!("Deserialized transaction: {:?}", transaction);

            //TODO - 2
            //add a logic to discard those tx which are of except create_token and thos tokens who's tx of creation we are now getting is to high 
        
            pub const CREATE_DISCRIMINATOR: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
            if transaction.get_data().starts_with(&CREATE_DISCRIMINATOR) && time::Instant::now().duration_since(transaction.get_timestamp()) < Duration::from_secs(60) {
                // Process the transaction
                let trade = transaction.place_trade();
                println!("Trade placed: {:?}", trade);
            } else {
                println!("Ignoring transaction with program ID: {}", transaction.get_program_id());
            }
        
        }
        Ok(())

    }
}
}
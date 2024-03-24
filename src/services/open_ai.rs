use std::io::{Error, ErrorKind::InvalidInput};

use async_openai::{config::OpenAIConfig, types::CreateEmbeddingRequestArgs, Client};
use tiktoken_rs::{cl100k_base, CoreBPE};

struct OpenAI {
    client: Client<OpenAIConfig>,
    tokenizer: CoreBPE,
}

impl Default for OpenAI {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenAI {
    pub fn new() -> Self {
        let client = Client::new();
        let tokenizer = cl100k_base().unwrap();

        OpenAI { client, tokenizer }
    }

    pub async fn string_to_embedding(&self, input: &str) -> Result<Vec<f32>, Error> {
        let tokens = self.tokenize_string(input)?;
        let embedding_request = CreateEmbeddingRequestArgs::default()
            .model("text-embedding-ada-002")
            .input(tokens)
            .build()
            .unwrap();
        let embedding = self
            .client
            .embeddings()
            .create(embedding_request)
            .await
            .unwrap();
        Ok(embedding.data[0].embedding.clone())
    }

    fn tokenize_string(&self, input: &str) -> Result<Vec<u32>, Error> {
        let tokens = self.tokenizer.encode_with_special_tokens(input);

        if tokens.len() > 8192 {
            return Err(Error::new(InvalidInput, "Input too long"));
        }

        Ok(tokens.iter().map(|&token| token as u32).collect())
    }
}

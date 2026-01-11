use llama_cpp::{LlamaModel, LlamaParams, SessionParams};
use llama_cpp::standard_sampler::StandardSampler;

pub struct GhostBrain {
    model: LlamaModel,
}

impl GhostBrain {
    pub fn new(model_path: &str) -> Self {
        let params = LlamaParams::default();
        let model = LlamaModel::load_from_file(model_path, params)
            .expect("❌ Model not found! Ensure ghost-brain.gguf is in the /models folder.");
        
        Self { model }
    }

    pub fn generate_advice(&self, context: &str) -> String {
        let mut session = self.model.create_session(SessionParams::default()).unwrap();
        
        let prompt = format!(
            "System: You are a terminal helper. Respond with ONLY the fixed command and NO explanation.\nUser Error: {}\nAssistant: Fixed command:",
            context
        );

        let _ = session.advance_context(&prompt);
        
        let completions = session
            .start_completing_with(StandardSampler::default(), 20)
            .expect("Failed to start completion")
            .into_strings();

        let mut response = String::new();
        for chunk in completions {
            response.push_str(&chunk);
        }

        // Filter out emojis and non-text junk
        let cleaned: String = response.chars().filter(|c| c.is_ascii()).collect();
        let final_text = cleaned.trim().to_string();

        if final_text.is_empty() {
            "Check syntax.".to_string()
        } else {
            final_text
        }
    }
}

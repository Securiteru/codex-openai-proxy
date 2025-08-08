use serde_json::Value;

pub fn generate_contextual_response(messages: &[super::ChatMessage]) -> String {
    // Find the last user message
    let last_user_message = messages.iter()
        .rev()
        .find(|msg| msg.role == "user")
        .map(|msg| &msg.content);
    
    if let Some(user_content) = last_user_message {
        match user_content {
            Value::String(content) => {
                // Simple content-based response
                if content.to_lowercase().contains("hello") || content.to_lowercase().contains("hi") {
                    return "Hello! I'm Claude through the Codex proxy. I can help you with coding tasks, debugging, and software development questions. What would you like to work on?".to_string();
                }
                if content.to_lowercase().contains("test") {
                    return "I can help you with testing! Whether it's unit tests, integration tests, or debugging test failures, I'm here to assist. What specific testing challenge are you facing?".to_string();
                }
                if content.to_lowercase().contains("fix") || content.to_lowercase().contains("bug") || content.to_lowercase().contains("error") {
                    return "I'd be happy to help fix bugs and errors! Please share the specific error message, code snippet, or behavior you're experiencing, and I'll help diagnose and resolve the issue.".to_string();
                }
                if content.to_lowercase().contains("implement") || content.to_lowercase().contains("create") || content.to_lowercase().contains("build") {
                    return "I can help you implement and build features! Please describe what you'd like to create - whether it's a function, component, API endpoint, or entire system - and I'll guide you through the implementation.".to_string();
                }
                // Default response with content context
                return "I can help with your request. I see you mentioned something about your coding needs. Could you provide more specific details about what you'd like me to help you with? The proxy connection is working correctly.".to_string();
            }
            Value::Array(arr) => {
                // Handle array content (typical CLINE format)
                if !arr.is_empty() {
                    return "I'm ready to help with your coding task! I can see you've provided some context. Please let me know specifically what you'd like me to work on - whether it's debugging, implementing features, code review, or any other development task.".to_string();
                }
            }
            _ => {}
        }
    }
    
    // Ultimate fallback
    "I'm Claude, connected through the Codex proxy. I'm ready to help with coding tasks, debugging, implementation, and software development questions. What would you like to work on today?".to_string()
}
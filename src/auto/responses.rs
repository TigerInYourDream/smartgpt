use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::{Message, AgentInfo};

#[derive(Serialize, Deserialize)]
pub struct Response {
    response: String
}

#[allow(dead_code)]
pub fn create_runner_prompt() -> String {
    r#"Now, please write a response back to the user. Tell the user, in detail, everything you did, the outcome, and any permanent changes that were carried out."#.to_string()
}

#[allow(dead_code)]
pub fn ask_for_responses(agent: &mut AgentInfo) -> Result<String, Box<dyn Error>> {
    agent.llm.message_history.push(Message::User(create_runner_prompt()));

    let response = agent.llm.model.get_response_sync(
        &agent.llm.get_messages(), Some(1000), None
    )?;

    Ok(response)
}
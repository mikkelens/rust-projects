use std::io;

use anyhow::Ok;
use anyhow::Result;

use async_openai::Chat;
use async_openai::Client;
use async_openai::types::ChatCompletionRequestMessage;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_openai::types::Role;
use linefeed::DefaultTerminal;
use linefeed::Interface;
use linefeed::ReadResult;

pub async fn talk(reader: &Interface<DefaultTerminal>) -> Result<()> {
    let client = Client::new();
    
    let chat = Chat::new(&client);
    let mut messages: Vec<ChatCompletionRequestMessage>  = vec![];
    let mut default_args = CreateChatCompletionRequestArgs::default();
    let request_args = default_args
        .model("gpt-3.5-turbo")
        .user("async-openai")
        // .stream(true)
        .n(1);
    
    println!("[Type 'exit' to leave conversation.]");
    
    while let Some(user_input) = parse_exitable(reader.read_line()) {
        messages.push(ChatCompletionRequestMessage { role: Role::User, content: user_input, name: None });
        let request = request_args.messages(messages.clone()).build()?;
        let response = chat.create(request).await?;
        // let message = response.choices.first()?;
        messages.push(ChatCompletionRequestMessage { role: Role::Assistant, content: message, name: None });
        
    } // run untill we should exit
    
    Ok(())
}

fn parse_exitable(result: io::Result<ReadResult>) -> Option<String> {
    let ReadResult::Input(line) = result.ok()? else {
        return None;
    };
    if line.contains("exit") {
        return None;
    }
    
    Some(line)   
}
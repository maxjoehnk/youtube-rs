use youtube_api::YoutubeApi;
use youtube_api::auth::stdio_login;
use std::thread;
use std::time;

/// This example requires the "post" feature enabled otherwise it will
/// return an error when it tries to post to the chat
#[tokio::main]
async fn main() {
    let api = YoutubeApi::new_with_oauth(env!("YOUTUBE_API_KEY").to_string(),
        env!("YOUTUBE_CLIENT_ID").to_string(),
        env!("YOUTUBE_CLIENT_SECRET").to_string(),None).unwrap();
    
    api.login(stdio_login).await.unwrap();

    // Assumes you have a single live broadcast running
    let broadcasts = api.list_live_broadcasts(
                youtube_api::models::LiveBroadcastRequestBuilder{max_results:Some(10),mine:Some(true)})
            .await.unwrap();
    let chat = &broadcasts.items[0].snippet.content_details.live_chat_id;

    // Read all the messages in chat and ignore them
    let thing = api.get_live_chat(
                youtube_api::models::LiveChatRequestBuilder{live_chat_id:Some(chat.to_string()),page_token:None})
            .await.unwrap();
    let mut next = thing.next_page_token;

    // Every 5 seconds read next set of messages (up to 500 messages)
    // scan them for "!ping" and respond with "pong <name of person who sent ping>"    
    loop {
        let thing = api.get_live_chat(
                youtube_api::models::LiveChatRequestBuilder{
                    live_chat_id:Some(chat.to_string()),page_token:next
                })
            .await.unwrap();
        next = thing.next_page_token;
        for (cmd,name) in thing.items.iter()
                            .map(|x|(&x.snippet.content_details.display_message , &x.author_details.display_name))
                            .filter(|(x,_)|x.starts_with("!")) {
            match cmd.as_str() {
                "!ping" => {
                    match api.send_live_chat(
                        youtube_api::models::SendLiveChatRequestBuilder{live_chat_id:Some(chat.to_string())},
                        youtube_api::models::SendLiveChatRequestBody{
                            snippet:youtube_api::models::SendLiveChatRequestBodyContents{
                                live_chat_id:Some(chat.to_string()),
                                typed:Some("textMessageEvent".to_string()),
                                text_message_details:youtube_api::models::SendLiveChatMessageDetails{
                                    message_text:Some(format!("pong {}",name))
                                }
                                }}).await
                    {
                        Ok(_) => (),
                        Err(err) => println!("Send live chat failed: {:?}",err)
                    }
                },
                x => println!("unknown! {} ({})",x,name)
            }
        }
        thread::sleep(time::Duration::from_millis(1000));
    }
}

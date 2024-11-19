use crate::database::Sponsor;
use async_openai::types::{
    ChatCompletionRequestAssistantMessageContent, ChatCompletionRequestMessage,
    ChatCompletionRequestUserMessageContent,
};
use srtlib::{Subtitle, Subtitles, Timestamp};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct CachedCall {
    pub name: String,
    pub sponsor: Sponsor,
    pub start: Instant,
    pub messages: Vec<ChatCompletionRequestMessage>,
    pub timestamps: Vec<Timespan>,
}

#[derive(Debug, Clone)]
pub struct Timespan {
    pub start: Instant,
    pub end: Instant,
}

impl CachedCall {
    pub fn new(sponsor: Sponsor) -> Self {
        Self {
            sponsor,
            name: String::new(),
            start: Instant::now(),
            messages: Vec::new(),
            timestamps: Vec::new(),
        }
    }

    /// Adds a system message to the conversation cache with the current time as both start and end time.
    /// For an accurate timestamp `end_last_message` must be called before adding a new message.
    pub fn add_system_message(&mut self, message: ChatCompletionRequestMessage) {
        self.messages.push(message);
        self.timestamps.push(Timespan {
            start: Instant::now(),
            end: Instant::now(),
        });
    }

    /// Adds a user message to the conversation cache with the last message's end time
    /// as the start time of the new message and the current time as the end time.
    pub fn add_user_message(&mut self, message: ChatCompletionRequestMessage) {
        self.messages.push(message);
        self.timestamps.push(Timespan {
            start: self
                .timestamps
                .last()
                .map(|t| t.end)
                .unwrap_or_else(Instant::now),
            end: Instant::now(),
        });
    }

    /// Sets the end time of the last message to the current time.
    /// This is called when receiving a callback where the previous
    /// instruction was a twilio `Say` verb.
    pub fn end_last_message(&mut self) {
        if let Some(timestamp) = self.timestamps.last_mut() {
            timestamp.end = Instant::now();
        }
    }

    /// Collect all user and assistant messages together in a SRT file,
    /// which can be used to display subtitles with correct and accurate
    /// timestamps under the video recording of the call.
    pub fn write_subtitles_to_file(&self, call_id: &str) {
        let _ = std::fs::create_dir_all(format!("cache/recordings/{call_id}"));
        let mut subtitles = Subtitles::new();
        for (index, (message, timespan)) in
            self.messages.iter().zip(self.timestamps.iter()).enumerate()
        {
            let start =
                Timestamp::from_milliseconds((timespan.start - self.start).as_millis() as _);
            let end = Timestamp::from_milliseconds((timespan.end - self.start).as_millis() as _);
            if let Some(content) = Self::extract_message_content(message) {
                subtitles.push(Subtitle::new(index, start, end, content));
            }
        }

        let path = format!("cache/recordings/{call_id}/subtitles.srt");
        if let Err(e) = subtitles.write_to_file(&path, None) {
            log::error!("Failed to write subtitles to file: {e:?}");
        }
    }

    /// Extracts the content of a message from the openai chat completion if the
    /// message is either a user or assistant message with text content. System messages
    /// are ignored, as they are not part of the (audible) conversation.
    fn extract_message_content(message: &ChatCompletionRequestMessage) -> Option<String> {
        match message {
            ChatCompletionRequestMessage::User(user) => match user.content.to_owned() {
                ChatCompletionRequestUserMessageContent::Text(text) => Some(text),
                _ => None,
            },
            ChatCompletionRequestMessage::Assistant(assistant) => {
                match assistant.content.to_owned() {
                    Some(ChatCompletionRequestAssistantMessageContent::Text(text)) => Some(text),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

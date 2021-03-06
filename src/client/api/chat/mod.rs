use super::*;
use crate::{api::chat::*, client_api};

use hrpc::client::socket::Socket;

pub use crate::api::chat::EventSource;

/// A message location. This type can be used as multiple requests.
#[into_request("GetMessageRequest", "DeleteMessageRequest")]
#[derive(new, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageLocation {
    guild_id: u64,
    channel_id: u64,
    message_id: u64,
}

/// Wrapper around a guild ID which can be used as multiple requests.
#[into_request(
    "DeleteGuildRequest",
    "LeaveGuildRequest",
    "GetGuildRequest",
    "GetGuildInvitesRequest",
    "GetGuildChannelsRequest",
    "GetGuildRolesRequest",
    "GetGuildMembersRequest"
)]
#[derive(new, Debug, Clone, Copy, PartialEq, Eq, Into, From)]
pub struct GuildId {
    guild_id: u64,
}

/// Wrapper around an user ID which can be used as multiple requests.
#[into_request("GetUserRequest")]
#[derive(new, Debug, Clone, Copy, PartialEq, Eq, Into, From)]
pub struct UserId {
    user_id: u64,
}

/// Manage and query channels.
pub mod channel;
/// Manage and query emotes and emote packs.
pub mod emote;
/// Manage and query guilds.
pub mod guild;
/// Manage and query invites.
pub mod invite;
/// Manage and query messages.
pub mod message;
/// Manage and query user permissions and roles.
pub mod permissions;
/// Manage and query user profiles.
pub mod profile;

/// Convenience type to create a valid [`TriggerActionRequest`].
#[into_request("TriggerActionRequest")]
#[derive(Debug, Clone, new)]
pub struct TriggerAction {
    guild_id: u64,
    channel_id: u64,
    message_id: u64,
    action_id: String,
    action_data: String,
}

client_api! {
    /// Triggers the specified action.
    request: TriggerActionRequest,
    api_fn: trigger_action,
    service: chat,
}

/// Convenience type to create a valid [`TypingRequest`].
#[into_request("TypingRequest")]
#[derive(Debug, Clone, Copy, new)]
pub struct Typing {
    guild_id: u64,
    channel_id: u64,
}

client_api! {
    /// Notifies the server that the local user is typing.
    request: TypingRequest,
    api_fn: typing,
    service: chat,
}

/// Stream events from the server.
///
/// This endpoint requires authentication.
pub async fn stream_events(client: &Client) -> ClientResult<Socket<StreamEventsRequest, Event>> {
    use hrpc::IntoRequest;

    let mut req = ().into_request();
    if client.data.auth_status.lock().is_authenticated() {
        req = req.header(
            http::header::AUTHORIZATION,
            // This is safe on the assumption that servers will never send session tokens
            // with invalid-byte(s). If they do, they aren't respecting the protocol
            unsafe {
                http::HeaderValue::from_maybe_shared_unchecked(
                    client.data.token_bytes.lock().clone(),
                )
            },
        );
    }
    let response = client.chat().await.stream_events(req).await;
    #[cfg(debug_assertions)]
    tracing::debug!("Received response: {:?}", response);
    response.map_err(Into::into)
}

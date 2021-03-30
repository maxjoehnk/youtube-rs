# youtube-api

Provide an async api for the youtube api.

Supprts:
* search
* playlists
* livebroacasts (aka live streams)

## APIs
### Search

### Playlists

### livebroadcasts
Function is in beta, see [examples](examples/pingpong.rs)

Contians basic functionality to allow creationg of a chat bot.
To allow posting messages to a livebroadcast requires the "post" feature.

## Features
### readonly (default)
Uses the readonly scope

### post
Requires readonly and adds the ability to post to the livechat of a livebroadcast
using the force-ssl scope


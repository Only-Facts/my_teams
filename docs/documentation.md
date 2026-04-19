# MYTEAMS Protocol Specification

## 1. Status of this Memo

This document specifies the application protocol used by the `myteams_server` and `myteams_cli` programs.

This protocol is a text-based protocol running over TCP.

## 2. Purpose

The goal of the protocol is to support a collaborative communication application with:

* user login and logout
* direct private messages between users
* team subscription management
* hierarchical navigation through team, channel, and thread contexts
* creation and listing of teams, channels, threads, and replies
* server-generated event notifications

## 3. Transport

* Transport: TCP
* Encoding: UTF-8 text
* Message delimiter: newline (`\n`)
* One command or server response is carried per line.

The server processes commands line by line. Partial reads and writes are allowed at the transport level, but application messages are considered complete only once a newline is received.

## 4. Command Syntax

Each client request is a single line.

General form:

* `/command`
* `/command "arg1"`
* `/command "arg1" "arg2"`
* `/command "arg1" "arg2" "arg3"`

### 4.1 Quoting Rules

All command arguments can be enclosed in double quotes.

Examples:

* `/login alice`
* `/send "user_uuid" "hello"`
* `/use "team_uuid" channel_uuid`

Invalid examples:

* `/login "alice`
* `/send "user_uuid" "hello`
* `/create "team_name" desc"`
* `/use team_uuid"`

If a closing quote is missing, the server responds with:

* `400 Bad Request`

### 4.2 Length Constraints

The following limits apply:

* `MAX_NAME_LENGTH = 32`
* `MAX_DESCRIPTION_LENGTH = 255`
* `MAX_BODY_LENGTH = 512`

If a request exceeds these constraints, the server responds with a `400` error.

## 5. Client to Server Commands

### 5.1 `/help`

Displays client help.

This command is local to the CLI and does not require a server round trip.

### 5.2 `/login "user_name"`

Logs in an existing user by name, or creates the user if it does not already exist.

Successful response:

* `200 Login OK|user_uuid|user_name`

Errors:

* `400 Bad Request: Missing user_name`
* `400 Bad Request: Name too long`

### 5.3 `/logout`

Logs out the current user.

Successful response:

* `200 Logout OK`

### 5.4 `/users`

Lists all users in the domain.

Successful response:

* `200 USERS|user_uuid:user_name:status|...`

Where `status` is:

* `1` for connected
* `0` for disconnected

Errors:

* `401 Unauthorized: Please login first`

### 5.5 `/user "user_uuid"`

Displays information about a specific user.

Successful response:

* `200 USER|user_uuid|user_name|status`

Errors:

* `400 Bad Request`
* `401 Unauthorized`
* `404 Not Found: User not found`

### 5.6 `/send "user_uuid" "message_body"`

Sends a private message to another user.

Successful response:

* `200 Message Sent`

Server event sent to the receiver if connected:

* `EVENT PM_RECEIVED|sender_uuid|message_body`

Errors:

* `400 Bad Request: /send "user_uuid" "message"`
* `400 Bad Request: Message too long`
* `401 Unauthorized: Please login first`
* `404 Not Found: User does not exist`

### 5.7 `/messages "user_uuid"`

Lists all private messages exchanged with a specific user.

Successful response:

* `200 MESSAGES|sender_uuid:timestamp:message_body|...`

Errors:

* `400 Bad Request`
* `401 Unauthorized`
* `404 Not Found: User not found`

### 5.8 `/subscribe "team_uuid"`

Subscribes the current user to a team.

Successful response:

* `200 SUBSCRIBED|user_uuid|team_uuid`

Errors:

* `400 Bad Request`
* `401 Unauthorized`
* `404 Not Found: Team not found`

### 5.9 `/unsubscribe "team_uuid"`

Unsubscribes the current user from a team.

Successful response:

* `200 UNSUBSCRIBED|user_uuid|team_uuid`

Errors:

* `400 Bad Request`
* `401 Unauthorized`
* `401 Unauthorized: Not subscribed to team`
* `404 Not Found: Team not found`

If the user is currently in a context inside that team, the server resets the user context to global.

### 5.10 `/subscribed`

Lists all teams to which the current user is subscribed.

Successful response:

* `200 SUBSCRIBED_TEAMS|team_uuid:team_name:team_description|...`

Errors:

* `401 Unauthorized`

### 5.11 `/subscribed "team_uuid"`

Lists all users subscribed to a team.

Successful response:

* `200 SUBSCRIBED_USERS|user_uuid:user_name:status|...`

Errors:

* `400 Bad Request`
* `401 Unauthorized`
* `401 Unauthorized: Not subscribed to team`
* `404 Not Found: Unknown Team`

### 5.12 `/use`

Sets the current context to global.

Successful response:

* `200 Context Updated`

### 5.13 `/use "team_uuid"`

Sets the current context to a team.

Successful response:

* `200 Context Updated`

Errors:

* `401 Unauthorized: Please login first`
* `401 Unauthorized: Not subscribed to team`
* `404 Not Found: Unknown Team`

### 5.14 `/use "team_uuid" "channel_uuid"`

Sets the current context to a channel.

Successful response:

* `200 Context Updated`

Errors:

* `401 Unauthorized: Please login first`
* `401 Unauthorized: Not subscribed to team`
* `404 Not Found: Unknown Team`
* `404 Not Found: Unknown Channel`

### 5.15 `/use "team_uuid" "channel_uuid" "thread_uuid"`

Sets the current context to a thread.

Successful response:

* `200 Context Updated`

Errors:

* `401 Unauthorized: Please login first`
* `401 Unauthorized: Not subscribed to team`
* `404 Not Found: Unknown Team`
* `404 Not Found: Unknown Channel`
* `404 Not Found: Unknown Thread`

### 5.16 `/create`

Behavior depends on the current context.

#### Global context

Command:

* `/create "team_name" "team_description"`

Successful response:

* `200 TEAM_CREATED|team_uuid|team_name|team_description`

#### Team context

Command:

* `/create "channel_name" "channel_description"`

Successful response:

* `200 CHANNEL_CREATED|channel_uuid|channel_name|channel_description`

Server event broadcast to team subscribers:

* `EVENT CHANNEL_CREATED|channel_uuid|channel_name|channel_description`

#### Channel context

Command:

* `/create "thread_title" "thread_message"`

Successful response:

* `200 THREAD_CREATED|thread_uuid|user_uuid|timestamp|thread_title|thread_message`

Server event broadcast to team subscribers:

* `EVENT THREAD_CREATED|thread_uuid|user_uuid|timestamp|thread_title|thread_message`

#### Thread context

Command:

* `/create "comment_body"`

Successful response:

* `200 REPLY_CREATED|thread_uuid|user_uuid|timestamp|reply_body`

Server event broadcast to team subscribers:

* `EVENT THREAD_REPLY_RECEIVED|team_uuid|thread_uuid|user_uuid|reply_body`

General errors:

* `400 Bad Request`
* `400 Bad Request: Length error`
* `400 Bad Request: Reply too long`
* `401 Unauthorized`
* `401 Unauthorized: Not subscribed to team`
* `404 Not Found: Unknown Team`
* `404 Not Found: Unknown Channel`
* `404 Not Found: Unknown Thread`
* `409 Conflict: Team already exists`
* `409 Conflict: Channel already exists`

### 5.17 `/list`

Behavior depends on the current context.

#### Global context

* `200 LIST_TEAMS|team_uuid:team_name:team_description|...`

#### Team context

* `200 LIST_CHANNELS|channel_uuid:channel_name:channel_description|...`

#### Channel context

* `200 LIST_THREADS|thread_uuid:user_uuid:timestamp:thread_title:thread_message|...`

#### Thread context

* `200 LIST_REPLIES|thread_uuid:user_uuid:timestamp:reply_body|...`

Errors:

* `401 Unauthorized: Please login first`
* `401 Unauthorized: Not subscribed to team`
* `404 Not Found: Unknown Team`
* `404 Not Found: Unknown Channel`
* `404 Not Found: Unknown Thread`

### 5.18 `/info`

Behavior depends on the current context.

#### Global context

* `200 INFO_USER|user_uuid|user_name|status`

#### Team context

* `200 INFO_TEAM|team_uuid|team_name|team_description`

#### Channel context

* `200 INFO_CHANNEL|channel_uuid|channel_name|channel_description`

#### Thread context

* `200 INFO_THREAD|thread_uuid|user_uuid|timestamp|thread_title|thread_message`

Errors:

* `401 Unauthorized`
* `401 Unauthorized: Not subscribed to team`
* `404 Not Found: Unknown Team`
* `404 Not Found: Unknown Channel`
* `404 Not Found: Unknown Thread`

## 6. Server to Client Events

The server may asynchronously send event lines to connected clients.

### 6.1 Private message received

* `EVENT PM_RECEIVED|sender_uuid|message_body`

### 6.2 Channel created in subscribed team

* `EVENT CHANNEL_CREATED|channel_uuid|channel_name|channel_description`

### 6.3 Thread created in subscribed team

* `EVENT THREAD_CREATED|thread_uuid|user_uuid|timestamp|thread_title|thread_message`

### 6.4 Reply created in subscribed team

* `EVENT THREAD_REPLY_RECEIVED|team_uuid|thread_uuid|user_uuid|reply_body`

## 7. Error Model

The protocol uses text status lines.

Common status prefixes:

* `200` success
* `400` malformed request or invalid syntax
* `401` unauthorized access
* `404` unknown resource
* `409` conflict on creation

## 8. Context Model

Each connected client has one current context:

* Global
* Team
* Channel
* Thread

The current context affects the behavior of:

* `/create`
* `/list`
* `/info`

Context is updated using `/use`.

When a user unsubscribes from a team, any active context inside that team is reset to global.

## 9. Security Rules

The following rules apply:

* A client MUST be logged in before using protected commands.
* A client MUST be subscribed to a team before entering that team context.
* A client MUST be subscribed to a team before listing or creating resources inside it.
* A client MUST NOT receive events from teams to which it is not subscribed.

## 10. Persistence Model

The server persists the following information on shutdown and restores it on startup:

* users
* teams
* team subscriptions
* channels
* threads
* replies
* private messages

The persistence format is internal to the server implementation and not part of the network protocol.

## 11. Example Session

### 11.1 Login

Client:

* `/login "alice"`

Server:

* `200 Login OK|18a7c3ce493c6578|alice`

### 11.2 Create team

Client:

* `/create "team1" "my first team"`

Server:

* `200 TEAM_CREATED|18a7d100abcd1234|team1|my first team`

### 11.3 Subscribe and enter team

Client:

* `/subscribe "18a7d100abcd1234"`
* `/use "18a7d100abcd1234"`

Server:

* `200 SUBSCRIBED|18a7c3ce493c6578|18a7d100abcd1234`
* `200 Context Updated`

### 11.4 Create channel

Client:

* `/create "general" "main discussion"`

Server:

* `200 CHANNEL_CREATED|18a7d155ffff2222|general|main discussion`

Subscribers receive:

* `EVENT CHANNEL_CREATED|18a7d155ffff2222|general|main discussion`

### 11.5 Create thread

Client:

* `/use "18a7d100abcd1234" "18a7d155ffff2222"`
* `/create "welcome" "hello everyone"`

Server:

* `200 Context Updated`
* `200 THREAD_CREATED|18a7d1a0eeee3333|18a7c3ce493c6578|1713535200|welcome|hello everyone`

Subscribers receive:

* `EVENT THREAD_CREATED|18a7d1a0eeee3333|18a7c3ce493c6578|1713535200|welcome|hello everyone`

### 11.6 Reply

Client:

* `/use "18a7d100abcd1234" "18a7d155ffff2222" "18a7d1a0eeee3333"`
* `/create "first reply"`

Server:

* `200 Context Updated`
* `200 REPLY_CREATED|18a7d1a0eeee3333|18a7c3ce493c6578|1713535300|first reply`

Subscribers receive:

* `EVENT THREAD_REPLY_RECEIVED|18a7d100abcd1234|18a7d1a0eeee3333|18a7c3ce493c6578|first reply`

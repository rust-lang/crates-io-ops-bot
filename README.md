# ops-bot
a bot to help assist the crates.io, website, and other rust ops teams

[![Build Status](https://travis-ci.com/rust-lang/ops-bot.svg?token=hHeDp9pQmz9kvsgRNVHy&branch=master)](https://travis-ci.com/rust-lang/ops-bot)

**This project is currently under development**

## Intro

This is a chat bot for [Discord](https://discordapp.com/) to manage the infrastructure of [crates.io](https://crates.io/).

Most of the infrastructure for crates.io is hosted by [Heroku](https://www.heroku.com/). Currently if someone
is to manage any part of the infrastructure for crates.io, they must have full credentials to Heroku. 

The purpose of this bot is to allow basic infrastructure management tasks to be conducted through the Rust Language Discord server.
It would be limited to specific members of the crates.io team, but it would allow more people to assist with basic infrastructure
management without needing full credentials to Heroku.

### Stage 1

(Still in progress)

Stage 1 of this bot will allow an authorized Discord member to:
* Update select environmental variables (not including things like API keys)
* Restart the application (which fixes most issues)
* Scale the application
* Change Dyno types
* Deploy the application
* Rollback a deployment
* Block/unblock ips

### Future Stages

To be determined!

## Usage

This Discord bot allows you to run commands in a Discord channel like this:

```
~command
```

### Configuring Commands

By default, all commands are locked down and only can be run by users with ids
in the AUTHORIZED_USERS environmental variable. 

If you would like a command to be runnable by anyone (not just those
defined in AUTHORIZED USERS), you need to add it to the NO_AUTH_COMMANDS constant.

**lib.rs**
```rust
// These commands do not require a user
// to be in the AUTHORIZED_USERS env variable
const NO_AUTH_COMMANDS: &[&str] = &["ping", "multiply", "myid"]
```

### General Commands

For example, if you run the ~ping command is a Discord channel, this bot will respond with "Pong!"

```
you: ~ping
crates-io-bot: @you:Pong!
```

If you run the ~multiply command with two numbers, this bot will respond with the product of the two numbers.

```
you: ~multiply 2 2
crates-io-bot: 4
```

If you run the ~myid command, this bot will respond with your
Discord account ID.

```
you: ~myid
crates-io-bot: @you: Here is your user id 1234567
```

### Heroku commands

**~get_app**

If you run the ~get_app command and pass it either the app name 
or the app id (and you have a Heroku token set in your environmental variables),
this bot will respond with information about that app

```
you: ~get_app your_app_name_or_id
crates-io-bot:
App ID: "123abc"
App Name: "My app 1"
Released At: 2020-02-12T00:35:44Z
Web URL: https://www.your_app.herokuapp.com

Formations for this app:

Name: web
Command: npm start
Quantity: 1
Size: Free
```

**~get_apps**

If you run the ~get_apps command (and you have a Heroku token set in your environmental variables),
this bot will respond with a list of apps associated with that Heroku account

```
you: ~get_apps
crates-io-bot: @you: Here are your Heroku apps
App ID: "123abc"
App Name: "My app 1"
Released At: 2020-02-12T00:35:44Z
Web URL: https://www.your_app.herokuapp.com

App ID: "456def"
App Name: "My app 2"
Released At: 2020-02-12T00:35:44Z
Web URL: https://www.your_app.herokuapp.com

App ID: "789ghi"
App Name: "My app 3"
Released At: 2020-02-12T00:35:44Z
Web URL: https://www.your_app.herokuapp.com
```

**~restart_app**

If you run the ~restart_app command and pass it either the app name 
or the app id (and you have a Heroku token set in your environmental variables),
this bot will send a request to restart all dynos associated with the app

```
you: ~restart_app your_app_name_or_id
crates-io-bot: @you: All dynos in your-app-name have been restarted.
```

**~update_app_config**

You can update authorized application configuration variables through the ~update_app_config command.

Authorized configuration variables are defined here:
**heroku.rs**
```rust
// Config variables that can be updated through Discord
const AUTHORIZED_CONFIG_VARS: &[&str] = &["FOO"];
```

Let's say you have an app called "testing-nell-bot". That app has a config variable with the key "FOO" and you want to update the value of that key to "bar". You would run this command:

```
you: ~update_app_config testing-nell-bot FOO bar
crates-io-bot: @you: Config Var has been updated {"FOO": "bar"}
```

**~get_app_releases**

You can get a list of releases for your app through the ~get_app_releases command.

```
you: ~get_app_releases testing-nell-bot
crates-io-bot: @you Here are your app releases
ID: abc-123
Version: 1
Status: succeeded

ID: def-456
Version: 2
Status: succeeded

ID: ghi-789
Version: 3
Status: succeeded
```

**~deploy_app**

If you would like to deploy your application, you can use this command (you can pass in the branch name, the commit id, or the full sha for the commit you want to deploy)

```
you: ~deploy_app app_name branch_commit_id_or_sha
```

For example: 

```
~deploy_app testing-nell-app master
crate-io-bot: @you Build in progress for testing-nell-app (this will take a few minutes)
Build ID is a30c6830-7e47-47ce-9f8d-1a883e4a9beb
Build a30c6830-7e47-47ce-9f8d-1a883e4a9beb is still pending...
Build a30c6830-7e47-47ce-9f8d-1a883e4a9beb is still pending...
Build a30c6830-7e47-47ce-9f8d-1a883e4a9beb is still pending...
Build a30c6830-7e47-47ce-9f8d-1a883e4a9beb is still pending...
Build a30c6830-7e47-47ce-9f8d-1a883e4a9beb is still pending...
@you: Build a30c6830-7e47-47ce-9f8d-1a883e4a9beb is complete for testing-nell-app, moving on to releasing the app
@you: App testing-nell-app version 0.2.1 has successfully been released!
```

This command will:
* Create a build of the code
* Provide updates on the build while it is in progress (this is configurable through the BUILD_CHECK_INTERVAL environmental variable)
* Release the application as the version you specified

**~rollback_app**

If you would like to rollback your app to the code associated with a previous release of your app, you can do so with the ~rollback_app command.

```
you: ~rollback_app testing-nell-bot version-to-rollback-to
```

```
you: ~rollback_app testing-nell-bot v5

crates-io-bot: @you App testing-nell-bot was successfully rolled back to the code at v5
```

You can either specify the version with a "v" before the version number or with just the number. This command will also work.

```
you: ~rollback_app testing-nell-bot 5

crates-io-bot: @you App testing-nell-bot was successfully rolled back to the code at 5
```

**~scale_app**

You can scale formations of dynos within your application through the ~scale_app command.

For example: Let's say you have an application ("testing-nell-bot") that is running
* 1 formation - called "web"
* with 2 dynos in that formation
* each dyno is size "standard-1X"

If you want to update the formation to have a total of 3 dynos in it, you would run this command:

```
~scale_app testing-nell-bot web 3 standard-1X
crates-io-bot: : App testing-nell-bot's formation web has been updated
Name: web
Command: npm start
Quantity: 3
Size: standard-1X
```

If you want to scale down the formation to have a total of 2 dynos in it, you would run this command:

```
~scale_app testing-nell-bot web 2 standard-1X
crates-io-bot: : App testing-nell-bot's formation web has been updated
Name: web
Command: npm start
Quantity: 2
Size: standard-1X
```

If you want to change the size of all dynos in a formation (for example, upgrading all dynos from "standard-1X" to "standard-2X"), you would run this command:

```
~scale_app testing-nell-bot web 2 standard-2X
crates-io-bot: : App testing-nell-bot's formation web has been updated
Name: web
Command: npm start
Quantity: 2
Size: standard-2X
```

**~block_ip**

If you wish to block an IP address from accessing your application, you can do so with the ~block_ip command.

```
you: ~block_ip you_app_name ip_address_to_block
```

```
you: ~block_ip testing-nell-bot 123.456.68
crates-io-bot: @you IP address 123.456.68 
```

**~unblock_ip**

If you wish to unblock an IP address that was previously
blocked for your application you can do so with the ~unblock_ip command:

```
you: ~unblock_ip you_app_name ip_address_to_unblock
```

```
you: ~unblock_ip testing-nell-bot 123.456.68
crates-io-bot: @you IP address 123.456.789 has been unblocked
```

## Setup

To setup this Discord bot, you need:
* A [Discord Account](https://discordapp.com/)
* A [Discord Server](https://support.discordapp.com/hc/en-us/articles/204849977-How-do-I-create-a-server-)
* A [Heroku Account](https://www.heroku.com/)

Go ahead and clone this repo and cd into the directory:

```bash
git clone https://github.com/rust-lang/crates-io-ops-bot.git
cd crates-io-ops-bot
```

### Setting Up a Discord Application

To use this bot, you will need to set up a Discord application through the [Discorse Developer Portal](https://discordapp.com/developers/). 

[This blog post on Davao JS](https://medium.com/davao-js/2019-tutorial-creating-your-first-simple-discord-bot-47fc836a170b) has a good guide
on creating a Discord application and generating the token key. Skip to the "Generating Token Key" heading in the post and come back here
when you create the token.

Make sure to store the token somewhere safe! You will need it!

To use the token in development and testing, you need to add it to your .env file. 
To set that file up:

```bash
cp .env.sample .env
```

Then add the token to your .env file:

**.env**
```
DISCORD_TOKEN="<paste your token here>"
```

To use the token in production, make sure to set it wherever you define your environmental variables
for your production environment.

### Setting up Authorized Users

Currently, all commands except ~ping can be run by anyone in the Discord server running this bot.

~ping is a restricted command - only authorized users can run it.

You can enable a user to run restricted commands by adding their Discord user id to the AUTHORIZED_USERS environmental variable.

To use the list of authorized users in development and test environments, set the variable in your .env file

**.env**
```
AUTHORIZED_USERS="123,456"
```

To use the authorized user list in a CI/CD or production environment, make sure to set it wherever you define your environmental variables
for that environment.

### Setting up the Heroku API key

In order to use commands that call out to Heroku, you must set the HEROKU_API_KEY environmental variable.

To use the Heroku API key in development at test environments, set this variable in your .env file

**.env**
```
HEROKU_API_KEY="123abc"
```

To use the Heroku API key in a CI/CD or production environment, make sure to set it wherever you define your environmental variables
for that environment.

### Setting up the GitHub Configuration

The ~deploy_app command requires three GitHub related environmental variables to be set. This includes your GitHub org, the repo you want to deploy from, and a [GitHub Personal Access Token](https://help.github.com/en/github/authenticating-to-github/creating-a-personal-access-token-for-the-command-line).


To configure these variables for development and test environments, set these variables in your .env file.

**.env**
```
GITHUB_ORG="your-github-org"
GITHUB_REPO="your-github-repo"
GITHUB_TOKEN="your-github-personal-access-token"
```
To use these variables in a CI/CD or production environment, make sure to set them wherever you define your environmental variables
for that environment.

### Setting up the Build Check Interval

The ~deploy_app command kicks of a build of your application and periodically checks the build to see if it is still pending. Once it is no longer pending, it moves onto releasing the build. To configure the check interval for development and test environments, set this variable in your .env file

This will set the build check interval to **5 seconds**

**.env**
```
BUILD_CHECK_INTERVAL="5"
```

To use the build check interval in a CI/CD or production environment, make sure to set it wherever you define your environmental variables
for that environment.

### Running locally

You can run this bot in your local environment with this command (make sure you are in the directory for your copy of this repo)

```bash
cargo run
```

Once it is running, you will see the bot in the "online" list on your Discord Server. Try out the commands!

### Running with Docker

There is a Dockerfile within this repository to make it easy to build and run this bot within a Docker container (make sure you are in the directory for your copy of this repo)

```bash
docker build -t your_name/crates-io-ops-bot .
docker run -i -t your_name/crates-io-ops-bot
```

### Running in Heroku

You can also easily run this bot in Heroku.

[This blog post on Davao JS](https://medium.com/davao-js/v2-tutorial-deploy-your-discord-bot-to-heroku-part-2-9a37572d5de4) has a good guide to manually setting 
up a Discord bot in Heroku. Make sure you set the DISCORD_TOKEN and (if necessary) AUTHORIZED_USERS environmental variables for your Heroku application!
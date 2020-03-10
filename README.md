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
```

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

If you run the ~restart_app command and pass it either the app name 
or the app id (and you have a Heroku token set in your environmental variables),
this bot will send a request to restart all dynos associated with the app

```
you: ~restart_app your_app_name_or_id
crates-io-bot: @you: All dynos in your-app-name have been restarted.
```

There will be more commands specific to managing the crates.io infrastructure very soon.

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

### Running locally

You can run this bot in your local environment with this command (make sure you are in your copy of this repo)

```bash
cargo run
```

Once it is running, you will see the bot in the "online" list on your Discord Server. Try out the commands!

### Running in Heroku

You can also easily run this bot in Heroku.

[This blog post on Davao JS](https://medium.com/davao-js/v2-tutorial-deploy-your-discord-bot-to-heroku-part-2-9a37572d5de4) has a good guide to manually setting 
up a Discord bot in Heroku. Make sure you set the DISCORD_TOKEN and (if necessary) AUTHORIZED_USERS environmental variables for your Heroku application!
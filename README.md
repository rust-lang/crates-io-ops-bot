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
# Example AWS ECS Terraform Config

Want to deploy this bot in AWS ECS? Check out this sample/reference terraform config!

**This config is meant for example/reference purposes only, the actual terraform code that is used to deploy the production instance for crates.io lives in [rust-lang/simpleinfra](https://github.com/rust-lang/simpleinfra/).**

## Pre-reqs

To run this terraform config, you must have:
* Terraform installed on your local system
* An AWS account
* AWS credentials set on your system (you can do this through the aws cli and `aws configure`)

## Setting up

First, clone this repo and cd into the terraform_example directory:

```bash
$ git clone https://github.com/rust-lang/crates-io-ops-bot/
$ cd crates-io-ops-bot/terraform_example
```

Now, copy the example variable values file:

```bash
$ cp example.tfvars terraform.tfvars
```

Then fill in terraform.tfvars with the appropriate values for your configuration of the bot.

Now, run terraform plan to preview the infrastructure which will be spun up:

```bash
$ terraform plan
```

And then, if all looks good, run terraform apply to spin the infra up!

```bash
$ terraform apply -var-file terraform.tfvars
```

It may take a few minutes for the containers to spin up. Once that is complete (you can verify through the AWS console), check out your Discord server you set up the bot in and it should be active!
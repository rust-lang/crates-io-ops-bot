variable "region" {
    type = string
    default = "us-west-2"
    description = "aws region to deploy to"
}

variable "access_key" {
    type = string
    description = "AWS Access Key ID"
}

variable "secret_key" {
    type = string
    description = "AWS Secret Key"
}
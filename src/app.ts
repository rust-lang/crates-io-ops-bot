import * as dotenv from "dotenv";
import * as Discord from "discord.js";
import { HerokuCommand } from "./heroku_command";

dotenv.config();
const client = new Discord.Client();
const authorizedUsers = process.env.AUTHORIZED_USERS.split(",");

let atMention: string;

client.on("ready", () => {
  console.log(`Logged in as ${client.user.tag}!`);
  atMention = `<@${client.user.id}>`;
});

client.on("message", async msg => {
  if (!isDmOrMentionInOpsChannel(msg)) {
    return;
  }

  const [command, ...args] = msg.content.replace(`${atMention} `, "").split(" ");

  if (!userCanExecuteCommand(msg.author, command, args)) {
    return;
  }

  console.log(`Received command from ${msg.author.tag} \`${msg.content}\``);

  if (command == "ping") {
    msg.channel.send("pong");
  } else if (command == "restart") {
    msg.channel.send("Running command");
    const cmd = HerokuCommand.run("restart");
    for await (const line of cmd.outputLines) {
      msg.channel.send(line);
    }
    if (await cmd.exitCode != 0) {
      msg.channel.send("Command failed");
    } else {
      msg.channel.send("Command complete");
    }
  }
});

client.login(process.env.DISCORD_TOKEN);

function isDmOrMentionInOpsChannel(msg: Discord.Message): boolean {
  return msg.channel.type === "dm" ||
    (<Discord.TextChannel> msg.channel).name === "crates-io-operations" && msg.content.startsWith(atMention);
}

function userCanExecuteCommand(user: Discord.User, command: string, args: string[]): boolean {
  return authorizedUsers.includes(user.id.toString());
}

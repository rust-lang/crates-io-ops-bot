import * as dotenv from "dotenv";
import * as Discord from "discord.js";

dotenv.config();
const client = new Discord.Client();
const authorizedUsers = process.env.AUTHORIZED_USERS.split(",");

let atMention: string;

client.on("ready", () => {
  console.log(`Logged in as ${client.user.tag}!`);
  atMention = `<@${client.user.id}>`;
});

client.on("message", msg => {
  if (!isDmOrMentionInOpsChannel(msg)) {
    return;
  }

  const [command, ...args] = msg.content.replace(`${atMention} `, "").split(" ");

  if (!userCanExecuteCommand(msg.author, command, args)) {
    return;
  }

  if (command == "ping") {
    msg.channel.send("pong");
  }
});

client.login(process.env.DISCORD_TOKEN);

function isDmOrMentionInOpsChannel(msg: Discord.Message): boolean {
  return msg.channel.type === "dm" ||
    (<Discord.TextChannel> msg.channel).name === "crates-io" && msg.content.startsWith(atMention);
}

function userCanExecuteCommand(user: Discord.User, command: string, args: string[]): boolean {
  return authorizedUsers.includes(user.id.toString());
}

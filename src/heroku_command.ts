import * as child from "child_process";
import * as util from "util";
import { ProcessLines } from "./process_lines";

export class HerokuCommand {
  private childProcess: child.ChildProcess;

  constructor(childProcess: child.ChildProcess) {
    this.childProcess = childProcess;
  }

  public static run(command: string, args: string[] = []): HerokuCommand {
    const fullArgs = [
      command,
      ...args,
      "-a",
      process.env.APP_NAME,
    ];
    const childProcess = child.spawn("heroku", fullArgs, {
      stdio: "pipe"
    });
    return new HerokuCommand(childProcess);
  }

  get outputLines(): AsyncIterable<string> {
    return new ProcessLines({
      stdout: this.childProcess.stdout,
      stderr: this.childProcess.stderr,
    });
  }

  get exitCode(): Promise<number> {
    return new Promise((resolve) => {
      this.childProcess.on("close", resolve);
    });
  }
}

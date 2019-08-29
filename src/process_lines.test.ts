import { ProcessLines } from "./process_lines";
import stream from "stream";
import assert from "assert";

describe("ProcessLines", () => {
  it("returns an async generator over the lines printed to stdout", async () => {
    const stdout = new stream.PassThrough();
    const processLines = new ProcessLines({ stdout });
    stdout.write("line");
    stdout.write("1\nline2\nl");
    stdout.end("ine3");

    const emittedLines = await iteratorToArray(processLines);
    expect(emittedLines).toEqual([
      "line1",
      "line2",
      "line3"
    ]);
  });

  it("returns an async generator over the lines printed to stderr", async () => {
    const stderr = new stream.PassThrough();
    const processLines = new ProcessLines({ stderr });
    stderr.write("errline");
    stderr.write("1\nerrline2\nerrl");
    stderr.end("ine3");

    const emittedLines = await iteratorToArray(processLines);
    expect(emittedLines).toEqual([
      "errline1",
      "errline2",
      "errline3"
    ]);
  });

  it("interleaves the output from stdout and stderr", async () => {
    const stdout = new stream.PassThrough();
    const stderr = new stream.PassThrough();
    const processLines = new ProcessLines({ stdout, stderr });
    stdout.write("line");
    stderr.write("errline1\nerr");
    stdout.end("1\nline2");
    stderr.end("line2");

    const emittedLines = await iteratorToArray(processLines);
    // This doesn't actually give the order we wrote the data, this just seems
    // to be a quirk of how node schedules work, and that it doesn't start
    // polling until after there's enough buffered for both generators to
    // resolve.
    //
    // Ultimately all we really care about is making sure the impl isn't
    // waiting for stdout to finish before showing stderr
    expect(emittedLines).toEqual([
      "line1",
      "errline1",
      "line2",
      "errline2",
    ]);
  });
});

async function iteratorToArray<T>(source: AsyncIterable<T>) {
  const items = [];
  for await (const item of source) {
    items.push(item);
  }
  return items;
}

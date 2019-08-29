import stream from "stream";

type stdio = {
  stdout?: stream.Readable,
  stderr?: stream.Readable,
};

export class ProcessLines {
  private stdout: stream.Readable;
  private stderr: stream.Readable;

  constructor({ stdout, stderr }: stdio) {
    this.stdout = stdout;
    this.stderr = stderr;
  }

  [Symbol.asyncIterator]() {
    return mixIterators(
      chunksToLines(this.stdout),
      chunksToLines(this.stderr),
    );
  }
}

async function* chunksToLines(chunks: AsyncIterable<string>) {
  if (!chunks) {
    return;
  }

  let currentChunk = "";

  for await (const chunk of chunks) {
    currentChunk += chunk;
    const lines = currentChunk.split("\n");
    currentChunk = lines.pop();
    for (const line of lines) {
      yield line;
    }
  }

  if (currentChunk != "") {
    yield currentChunk;
  }
}

async function* mixIterators(...iterators: AsyncIterator<string>[]) {
  async function nextWithIndex(iterator: AsyncIterator<string>, index: number) {
    return {
      index,
      result: await iterator.next(),
    };
  }

  const promises = iterators.map(nextWithIndex);
  let doneCount = 0;

  while (doneCount < iterators.length) {
    const { index, result: { done, value } } = await Promise.race(promises);
    if (done) {
      promises[index] = new Promise(() => {});
      doneCount += 1;
    } else {
      promises[index] = nextWithIndex(iterators[index], index);
      yield value;
    }
  }
}

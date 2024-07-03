import fs        from "node:fs";
import log       from "../../log.js";
import {Options} from "../../components/qualifyingRules.js";

export default function deleteKey(options: Options) {
  const key: string = options.targetKeys[0];
  log.debug(`Deleting "${key}"`);

  if (options.envObject[key]) {
    const lineStart = options.envObject[key].lineStart;
    const lineEnd   = options.envObject[key].lineEnd;
    log.debug(`Deleting lines ${lineStart}-${lineEnd}`);

    // Read the file and split it into an array of lines
    let lines: string[] = fs.readFileSync(options.fullEnvPath, 'utf8').split('\n');

    // Remove the lines between lineStart and lineEnd
    lines.splice(lineStart, lineEnd - lineStart + 1);

    // Join the lines back together and write the result back to the file
    fs.writeFileSync(options.fullEnvPath, lines.join('\n'));
  } else {
    log.debug(`Environment variable "${key}" not found`);
    process.exitCode = 1;
  }
};

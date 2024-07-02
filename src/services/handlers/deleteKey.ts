import log from "../../log.js";
import fs  from "node:fs";

import EnvObject from "../../envObject.js";

export default function deleteKey(envObject: EnvObject, envFilePath: string, key: string) {
  log.debug(`Deleting "${key}"`);

  if (envObject[key]) {
    const lineStart = envObject[key].lineStart;
    const lineEnd   = envObject[key].lineEnd;
    log.debug(`Deleting lines ${lineStart}-${lineEnd}`);

    // Read the file and split it into an array of lines
    let lines: string[] = fs.readFileSync(envFilePath, 'utf8').split('\n');

    // Remove the lines between lineStart and lineEnd
    lines.splice(lineStart, lineEnd - lineStart + 1);

    // Join the lines back together and write the result back to the file
    fs.writeFileSync(envFilePath, lines.join('\n'));
  } else {
    log.debug(`Environment variable "${key}" not found`);
    process.exitCode = 1;
  }
};

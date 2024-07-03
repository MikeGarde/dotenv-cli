import fs        from "node:fs";
import log       from "../../log.js";
import {Options} from "../../components/qualifyingRules.js";

export default function setValue(options: Options) {
  const key: string      = options.targetKeys[0];
  const newLines: string = `${key}=${options.setValue}`;

  log.debug(`Updating "${key}"`);

  // Do we want to update or append the .env file?
  if (options.envObject[key]) {
    log.debug('Updating existing key', options.envObject[key]);
    const lineStart = options.envObject[key].lineStart;
    const lineEnd   = options.envObject[key].lineEnd;
    log.debug(`Replacing lines ${lineStart}-${lineEnd}`);

    // Split the new lines into an array
    let newLinesArray: string[] = newLines.split('\n');

    // Read the file and split it into an array of lines
    let lines: string[] = fs.readFileSync(options.fullEnvPath, 'utf8').split('\n');

    // Replace the lines between lineStart and lineEnd
    lines.splice(lineStart, lineEnd - lineStart + 1, ...newLinesArray);

    // Join the lines back together and write the result back to the file
    fs.writeFileSync(options.fullEnvPath, lines.join('\n'));
  } else {
    log.debug(`Appending "${key}" to "${options.fullEnvPath}"`);

    fs.writeFileSync(options.fullEnvPath, `${newLines}\n`, {flag: 'a'});
  }
};

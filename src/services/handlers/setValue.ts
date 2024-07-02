import fs             from "node:fs";
import log            from "../../log.js";
import EnvObject      from "../../envObject.js";
import escapeAndQuote from "../../escapeAndQuote.js";

export default function setValue(envObject: EnvObject, envFilePath: string, key: string, setValue: string, quoteSet: boolean) {
  const newValue: string = escapeAndQuote(setValue, quoteSet);
  const newLines: string = `${key}=${newValue}`;

  log.debug(`Updating "${key}"`);

  // Do we want to update or append the .env file?
  if (envObject[key]) {
    log.debug('Updating existing key', envObject[key]);
    const lineStart = envObject[key].lineStart;
    const lineEnd   = envObject[key].lineEnd;
    log.debug(`Replacing lines ${lineStart}-${lineEnd}`);

    // Split the new lines into an array
    let newLinesArray: string[] = newLines.split('\n');

    // Read the file and split it into an array of lines
    let lines: string[] = fs.readFileSync(envFilePath, 'utf8').split('\n');

    // Replace the lines between lineStart and lineEnd
    lines.splice(lineStart, lineEnd - lineStart + 1, ...newLinesArray);

    // Join the lines back together and write the result back to the file
    fs.writeFileSync(envFilePath, lines.join('\n'));
  } else {
    log.debug(`Appending "${key}" to "${envFilePath}"`);

    fs.writeFileSync(envFilePath, `${newLines}\n`, {flag: 'a'});
  }
};

import fs            from 'node:fs';
import log           from './log.js';
import EnvObject     from "./envObject.js";
import EnvParseError from "./errors/EnvParseError.js";

function handleValue(value: string, line: number, startLine: number, envLines: string[], envObject: EnvObject, key: string, quote: string) {
  let multilineValue = value.slice(1);
  while (!multilineValue.trim().endsWith(quote)) {
    multilineValue += '\n' + envLines[++line];
  }
  envObject[key] = {
    value:     multilineValue.slice(0, -1),
    lineStart: startLine,
    lineEnd:   line
  };
}

/**
 * Parse the .env file into an object
 *
 * @param filePath
 */
function parseEnvFile(filePath: string): EnvObject {
  const envContent: string = fs.readFileSync(filePath, 'utf8');
  const envLines: string[] = envContent.split('\n');

  const envObject: EnvObject = new EnvObject();

  for (let line = 0; line < envLines.length; line++) {
    const trimmedLine: string = envLines[line].trim();
    const startLine: number   = line;

    if (!trimmedLine) {
      log.debug(`${line + 1} | Ignoring empty line`);
    } else if (trimmedLine.startsWith('#')) {
      log.debug(`${line + 1} | Ignoring comment`);
    } else {
      const [key, ...valueParts] = trimmedLine.split('=');
      const value: string        = valueParts.join('=').trim();

      if (key === trimmedLine) {
        log.debug(`${line + 1} | Ignoring line without key=value: ${envLines[line]}`);
        continue;
      }

      if (value.startsWith('"')) {
        log.debug(`${line + 1} | key: ${key}, double quoted, ${value.endsWith('"') ? 'single line' : 'multiline'}`);
        handleValue(value, line, startLine, envLines, envObject, key, '"');
      } else if (value.startsWith("'")) {
        log.debug(`${line + 1} | key: ${key}, single quoted, ${value.endsWith("'") ? 'single line' : 'multiline'}`);
        handleValue(value, line, startLine, envLines, envObject, key, "'");
      } else {
        log.debug(`${line + 1} | key: ${key}, un-quoted, single line`)
        if (value.includes('"') || value.includes("'")) {
          throw new EnvParseError(line + 1, `Invalid value: ${envLines[line]}`);
        }
        envObject[key] = {
          value:     value,
          lineStart: line,
          lineEnd:   line
        };
      }
    }
  }

  return envObject;
}

export default parseEnvFile;

export {EnvObject};

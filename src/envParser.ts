import fs            from 'node:fs';
import log           from './log.js';
import EnvObject     from "./envObject.js";
import ruleViolation from './ruleViolationError.js';

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

      if (value.startsWith('"') && value.endsWith('"')) {
        log.debug(`${line + 1} | key: ${key}, double quoted, single line`);
        //envObject[key] = value.slice(1, -1);
        envObject[key] = {
          value:     value.slice(1, -1),
          lineStart: startLine,
          lineEnd:   line
        };
      } else if (value.startsWith('"')) {
        log.debug(`${line + 1} | key: ${key}, double quoted, multiline`)
        let multilineValue = value.slice(1);
        while (!multilineValue.trim().endsWith('"')) {
          multilineValue += '\n' + envLines[++line];
        }
        envObject[key] = {
          value:     multilineValue.slice(0, -1),
          lineStart: startLine,
          lineEnd:   line
        };
      } else if (value.startsWith("'") && value.endsWith("'")) {
        log.debug(`${line + 1} | key: ${key}, single quoted, single line`);
        envObject[key] = {
          value:     value.slice(1, -1),
          lineStart: startLine,
          lineEnd:   line
        };
      } else if (value.startsWith("'")) {
        log.debug(`${line + 1} | key: ${key}, single quoted, multiline`)
        let multilineValue = value.slice(1);
        while (!multilineValue.trim().endsWith("'")) {
          multilineValue += '\n' + envLines[++line];
        }
        envObject[key] = {
          value:     multilineValue.slice(0, -1),
          lineStart: startLine,
          lineEnd:   line
        };
      } else {
        log.debug(`${line + 1} | key: ${key}, un-quoted, single line`)
        if (value.includes('"') || value.includes("'")) {
          // TODO: should we allow values that include closing quotes and escape them?
          throw new ruleViolation(`Invalid value on line ${line + 1}: ${envLines[line]}`);
        }
        //envObject[key] = value;
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

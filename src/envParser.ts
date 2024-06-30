import fs            from 'node:fs';
import log           from './log.js';
import ruleViolation from './ruleViolationError.js';

interface EnvObject {
  [key: string]: string;
}

function parseEnvFile(filePath: string): EnvObject {
  const envContent: string = fs.readFileSync(filePath, 'utf8');
  const envLines: string[] = envContent.split('\n');

  const envObject: EnvObject = {};

  for (let i = 0; i < envLines.length; i++) {
    const trimmedLine = envLines[i].trim();

    if (!trimmedLine) {
      log.debug('Ignoring empty line');
    } else if (trimmedLine.startsWith('#')) {
      log.debug('Ignoring comment');
    } else {
      const [key, ...valueParts] = trimmedLine.split('=');
      const value: string        = valueParts.join('=').trim();

      if (key === trimmedLine) {
        log.debug(`Ignoring line without key=value: ${envLines[i]}`);
        continue;
      }

      if (value.startsWith('"') && value.endsWith('"')) {
        log.debug(`key: ${key}, double quoted, single line`);
        envObject[key] = value.slice(1, -1);
      } else if (value.startsWith('"')) {
        log.debug(`key: ${key}, double quoted, multiline`)
        let multilineValue = value.slice(1);
        while (!multilineValue.endsWith('"')) {
          multilineValue += '\n' + envLines[++i];
        }
        envObject[key] = multilineValue.slice(0, -1);
      } else if (value.startsWith("'") && value.endsWith("'")) {
        log.debug(`key: ${key}, single quoted, single line`);
        envObject[key] = value.slice(1, -1);
      } else if (value.startsWith("'")) {
        log.debug(`key: ${key}, single quoted, multiline`)
        let multilineValue = value.slice(1);
        while (!multilineValue.endsWith("'")) {
          multilineValue += '\n' + envLines[++i];
        }
        envObject[key] = multilineValue.slice(0, -1);
      } else {
        log.debug(`key: ${key}, un-quoted, single line`)
        if (value.includes('"') || value.includes("'")) {
          // TODO: should we allow values that include closing quotes and escape them?
          throw new ruleViolation(`Invalid value on line ${i + 1}: ${envLines[i]}`);
        }
        envObject[key] = value;
      }
    }
  }

  return envObject;
}

export default parseEnvFile;

export {EnvObject};

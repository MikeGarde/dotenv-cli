import fs from 'node:fs';
import log from './log.js';

interface EnvObject {
  [key: string]: string;
}

function parseEnvFile(filePath: string): EnvObject {
  const envContent = fs.readFileSync(filePath, 'utf8');
  const envLines = envContent.split('\n');

  const envObject: EnvObject = {};

  for (let i = 0; i < envLines.length; i++) {
    const trimmedLine = envLines[i].trim();

    if (!trimmedLine) {
      log.debug('Ignoring empty line');
    } else if (trimmedLine.startsWith('#')) {
      log.debug('Ignoring comment');
    } else {
      const [key, ...valueParts] = trimmedLine.split('=');
      const value = valueParts.join('=').trim();

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
        envObject[key] = value;
      }
    }
  }

  return envObject;
}

export default parseEnvFile;

export { EnvObject };

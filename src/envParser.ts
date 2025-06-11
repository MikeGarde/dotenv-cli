import fs  from 'node:fs';
import log from './log.js';

import EnvObject, {EnvValue} from "./envObject.js";
import EnvParseError         from "./errors/EnvParseError.js";

function getEndLine(envLines: string[], startLine: number, quoteType: string): number {
  let endLine = startLine;
  while (!envLines[endLine].trim().endsWith(quoteType)) {
    endLine++;
  }
  return endLine;
}

function extractLines(envLines: string[], startLine: number, endLine: number, quoted: boolean): EnvValue {
  let allLines: string[] = envLines.slice(startLine, endLine + 1);
  allLines[0]            = allLines[0].split('=')[1];
  let blob: string       = allLines.join('\n').trim();

  if (quoted) {
    blob = blob.slice(1, -1);
  }

  if (blob.startsWith('[') && blob.endsWith(']')) {
    let arr: string[] = [];
    try {
      arr = JSON.parse(blob);
    } catch (e) {
      throw new EnvParseError(startLine + 1, `Invalid list: ${blob}`);
    }
    arr  = arr.map((item: string) => `"${item}"`);
    blob = '[' + arr.join(', ') + ']';
  }

  return {
    value:     blob,
    lineStart: startLine,
    lineEnd:   endLine
  };
}

function parseEnvFile(filePath: string): EnvObject {
  const envContent: string   = fs.readFileSync(filePath, 'utf8');
  const envLines: string[]   = envContent.split('\n');
  const envObject: EnvObject = new EnvObject();

  for (let lineCurrent = 0; lineCurrent < envLines.length; lineCurrent++) {
    const trimmedLine: string = envLines[lineCurrent].trim();
    const lineStart: number   = lineCurrent;

    if (!trimmedLine) {
      log.debug(`${lineCurrent + 1} | Ignoring empty line`);
    } else if (trimmedLine.startsWith('#')) {
      log.debug(`${lineCurrent + 1} | Ignoring comment`);
    } else {
      const [key, ...valueParts] = trimmedLine.split('=');
      const value: string        = valueParts.join('=').trim();

      if (key === trimmedLine) {
        log.debug(`${lineCurrent + 1} | Ignoring line without key=value: ${envLines[lineCurrent]}`);
        continue;
      }

      if (value.startsWith('"')) {
        log.debug(`${lineCurrent + 1} | key: ${key}, double quoted, ${value.endsWith('"') ? 'single line' : 'multiline'}`);
        lineCurrent    = getEndLine(envLines, lineStart, '"');
        envObject[key] = extractLines(envLines, lineStart, lineCurrent, true);
      } else if (value.startsWith("'")) {
        log.debug(`${lineCurrent + 1} | key: ${key}, single quoted, ${value.endsWith("'") ? 'single line' : 'multiline'}`);
        lineCurrent    = getEndLine(envLines, lineStart, "");
        envObject[key] = extractLines(envLines, lineStart, lineCurrent, true);
      } else if (value.startsWith('[')) {
        log.debug(`${lineCurrent + 1} | key: ${key}, list, ${value.endsWith(']') ? 'single line' : 'multiline'}`);
        lineCurrent    = getEndLine(envLines, lineStart, ']');
        envObject[key] = extractLines(envLines, lineStart, lineCurrent, false);
      } else {
        log.debug(`${lineCurrent + 1} | key: ${key}, un-quoted, single line`)

        const hasQuotes: boolean = value.includes('"') || value.includes("'");
        if (hasQuotes) {
          throw new EnvParseError(lineCurrent + 1, `Invalid value: ${envLines[lineCurrent]}`);
        }
        envObject[key] = extractLines(envLines, lineStart, lineCurrent, false);
      }
    }
  }

  envObject.resolveNestedVariables();

  return envObject;
}

export default parseEnvFile;

export {EnvObject};

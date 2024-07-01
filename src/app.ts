#!/usr/bin/env node

import {program}     from 'commander';
import formatValue   from './formatValue.js';
import fs            from 'node:fs';
import path          from 'node:path';
import parseEnvFile  from './envParser.js';
import RuleViolation from './ruleViolationError.js';
import * as url      from 'node:url';

import log, {setLogDebug} from './log.js';
import escapeAndQuote     from "./escapeAndQuote.js";

async function app() {
  const installDir: string  = path.dirname(url.fileURLToPath(import.meta.url));
  const packagePath: string = path.join(installDir, '../package.json');
  const packageJson         = JSON.parse(fs.readFileSync(packagePath, 'utf8'));

  // Parse command line options
  program
    .version(packageJson.version, '-v, --version', 'Output the version number')
    .helpOption('-h, --help', 'Show the help')
    .description('Read and update environment variables from a .env file')
    .argument('[key...]', 'Environment variable key')
    .option('-f, --file <filename>', 'Specify the .env file (default: .env)')
    .option('-j, --json', 'Output as JSON')
    .option('-m, --multiline', 'Allow multiline values')
    .option('-s, --set <value>', 'Update the environment variable in the .env file')
    .option('-q, --quote', 'Quote the value when --set regardless of need')
    .option('-d, --debug', 'Output extra debugging')
    .showSuggestionAfterError(true)
    .parse(process.argv);

  const options = program.opts();

  setLogDebug(options.debug);

  const envFilePath: string = options.file || '.env';
  const fullEnvPath: string = path.resolve(envFilePath);
  const keys: string[]      = program.args;
  const set: string         = options.set;

  // Multiple keys or no keys assume --json
  if (keys.length > 1 || !keys.length) {
    log.debug('Key count (0 or >1) defaulting to JSON');
    options.json = true;
  }
  log.debug('Keys:', keys);
  log.debug('Options:', options);
  log.debug('File:', fullEnvPath);

  const json: boolean      = (options.json !== undefined);
  const multiline: boolean = (options.multiline !== undefined);
  const quoteSet: boolean  = (options.quote !== undefined);

  // Qualifying Rules
  // - must have a .env file
  if (!fs.existsSync(envFilePath)) {
    throw new RuleViolation(`.env file not found: ${fullEnvPath}`);
  }
  // - cannot have both --json and --set
  if (json && set) {
    throw new RuleViolation('Cannot use --json and --set together');
  }
  // - must have a key if using --set
  if (set && (!keys.length || keys.length > 1)) {
    throw new RuleViolation('Must specify a single key when using --set');
  }
  // - cannot have both --json and --multiline
  if (json && multiline) {
    throw new RuleViolation('Cannot use --json and --multiline together');
  }

  let envObject = parseEnvFile(envFilePath);

  if (json && !keys.length) {
    log.debug('Outputting entire .env file as JSON');
    log.info(JSON.stringify(envObject));
  } else if (set) {
    const key: string      = keys[0];
    const newValue: string = escapeAndQuote(set, quoteSet);
    const line: string     = `${key}=${newValue}`;

    log.debug(`Updating "${key}"`);

    // Do we want to update or append the .env file?
    if (envObject[key]) {
      log.debug(`Replacing "${key}" in "${envFilePath}"`);

      const regex: RegExp = new RegExp(`${key}=.+`);
      const data: string  = fs.readFileSync(envFilePath, 'utf8').replace(regex, line);
      fs.writeFileSync(envFilePath, data);
    } else {
      log.debug(`Appending "${key}" to "${envFilePath}"`);

      fs.writeFileSync(envFilePath, `${line}\n`, {flag: 'a'});
    }
  } else {
    let result: string = '';

    for (const key of keys) {
      log.debug(`Getting "${key}"`);

      let value = formatValue(envObject[key], multiline);
      if (!value) {
        log.debug(`Environment variable "${key}" not found`);
        process.exitCode = 1;
      }
      value = json ? (value ? `"${value}"` : 'null') : value;
      result += json ? `"${key}": ${value},` : `${value}\n`;
    }

    // Removes trailing newline or comma
    result = result.slice(0, -1);
    if (json) {
      result = `{${result}}`;
    }
    log.info(result);
  }
}

app().then(() => {
  log.debug('done');
}).catch((error) => {
  if (error instanceof RuleViolation) {
    log.error(error.message);
  } else {
    log.error('An unexpected error occurred:', error);
  }
  process.exitCode = 1;
});

export default app;

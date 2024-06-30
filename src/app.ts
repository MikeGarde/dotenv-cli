#!/usr/bin/env node

import {program}     from 'commander';
import formatValue   from './formatValue.js';
import fs            from 'node:fs';
import path          from 'node:path';
import parseEnvFile  from './envParser.js';
import RuleViolation from './ruleViolationError.js';
import * as url      from 'node:url';

import log, {setLogDebug} from './log.js';

async function app() {
  // Get package.json
  const currentDirectory: string = path.dirname(url.fileURLToPath(import.meta.url));
  const packagePath: string      = path.join(currentDirectory, '../package.json');
  const packageJson              = JSON.parse(fs.readFileSync(packagePath, 'utf8'));

  // Parse command line options
  program
    .version(packageJson.version, '-v, --version', 'Output the version number')
    .helpOption('-h, --help', 'Show the help')
    .description('Read and update environment variables from a .env file')
    .argument('[key]', 'Environment variable key')
    .option('-f, --file <filename>', 'Specify the .env file (default: .env)')
    .option('-j, --json', 'Output as JSON')
    .option('-s, --set <value>', 'Update the environment variable in the .env file')
    .option('-m, --multiline', 'Allow multiline values')
    .option('-d, --debug', 'Output extra debugging')
    .showSuggestionAfterError(true)
    .parse(process.argv);

  const options = program.opts();

  setLogDebug(options.debug);

  const envFilePath: string = options.file || '.env';
  const fullEnvPath: string = path.resolve(envFilePath);
  const key: string         = program.args[0];
  const set: string         = options.set;
  const json: boolean       = (options.json !== undefined);
  const multiline: boolean  = (options.multiline !== undefined);

  // Qualifying Rules
  // - must have a .env file
  if (!fs.existsSync(envFilePath)) {
    throw new RuleViolation(`.env file not found: ${fullEnvPath}`);
  }
  // - cannot have both --json and --set
  if (json && set) {
    throw new RuleViolation('Cannot use --json and --set together');
  }
  // - must have a key if not using --json
  if (!json && !key) {
    throw new RuleViolation('Must specify a key');
  }
  // - must have a key if using --set
  if (set && !key) {
    throw new RuleViolation('Must specify a key when using --set');
  }
  // - cannot have both --json and --multiline
  if (json && multiline) {
    throw new RuleViolation('Cannot use --json and --multiline together');
  }

  let envObject = parseEnvFile(envFilePath);

  if (json && !key) {
    log.debug('Outputting entire .env file as JSON');
    log.info(JSON.stringify(envObject));
  } else if (set) {
    const value: string = set;
    const line: string  = `${key}=${value}`;

    log.debug(`Updating "${line}" in "${envFilePath}"`);

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
    log.debug(`Reading "${key}" from "${envFilePath}"`);

    const value = formatValue(envObject[key], multiline);
    if (value) {
      if (json) {
        log.info(`{"${key}": "${value}"}`);
      } else {
        log.info(value);
      }
    } else {
      log.debug(`Environment variable "${key}" not found in "${envFilePath}"`);
      process.exitCode = 1;
    }
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

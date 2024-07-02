#!/usr/bin/env node

import {program}    from 'commander';
import fs           from 'node:fs';
import path         from 'node:path';
import * as url     from 'node:url';
import parseEnvFile from './envParser.js';
import handlers     from "./services/handlers.js";

import log, {setLogDebug} from './log.js';
import readPipe           from "./readPipe.js";
import RuleViolationError from './errors/RuleViolationError.js';

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
    .option('-D, --delete', 'Delete the environment variable from the .env file')
    .option('-d, --debug', 'Output extra debugging')
    .showSuggestionAfterError(true)
    .parse(process.argv);

  const options = program.opts();

  setLogDebug(options.debug);

  const stdin: string | void = await readPipe().catch((err) => {
    throw new RuleViolationError(`Error reading from stdin: ${err}`);
  });

  const envFilePath: string = options.file || '.env';
  const fullEnvPath: string = path.resolve(envFilePath);
  const keys: string[]      = program.args;
  const set: string         = options.set;

  // Multiple keys or no keys assume --json
  if (keys.length > 1 || !keys.length) {
    log.debug('Key count (0 or >1) defaulting to JSON');
    options.json = true;
  }

  // Determine if we are setting a value, and if so, what's the value
  let setValue: string = '';
  if (stdin && set) {
    // - cannot have both --set [value] and stdin
    throw new RuleViolationError('Cannot use --set and stdin together');
  } else if (stdin) {
    setValue = stdin;
  } else if (set) {
    setValue = set;
  }

  log.debug('Keys:', keys);
  log.debug('Options:', options);
  log.debug('File:', fullEnvPath);

  const json: boolean      = (options.json !== undefined);
  const multiline: boolean = (options.multiline !== undefined);
  const quoteSet: boolean  = (options.quote !== undefined);
  const deleteKey: boolean = (options.delete !== undefined);
  const singleKey: boolean = (keys.length === 1);

  // Qualifying Rules
  // - must have a .env file
  if (!fs.existsSync(envFilePath)) {
    throw new RuleViolationError(`.env file not found: ${fullEnvPath}`);
  }
  // - cannot have both --json and --set
  if (json && setValue) {
    throw new RuleViolationError('Cannot use --json and --set together');
  }
  // - must have a key if using --set
  if (setValue && !singleKey) {
    throw new RuleViolationError('Must specify a single key when using --set');
  }
  // - cannot have both --json and --multiline
  if (json && multiline) {
    throw new RuleViolationError('Cannot use --json and --multiline together');
  }
  // - cannot use --delete with any other options
  if (deleteKey && (setValue || json || multiline)) {
    throw new RuleViolationError('Cannot use --delete with any other options');
  }
  // - must have a key if using --delete
  if (deleteKey && !singleKey) {
    throw new RuleViolationError('Must specify a single key when using --delete');
  }

  let envObject = parseEnvFile(envFilePath);

  if (json && !keys.length) {
    log.debug('Outputting entire .env file as JSON');
    log.info(envObject.toJsonString());
  } else if (deleteKey) {
    handlers.deleteKey(envObject, envFilePath, keys[0]);
  } else if (setValue) {
    handlers.setValue(envObject, envFilePath, keys[0], setValue, quoteSet);
  } else {
    handlers.getValue(envObject, keys, json, multiline);
  }
}

app().then(() => {
  log.debug('done');
}).catch((error) => {
  if (error instanceof RuleViolationError) {
    log.error(error.message);
  } else {
    log.error('An unexpected error occurred:', error);
  }
  process.exitCode = 1;
});

export default app;

#!/usr/bin/env node

import {program}    from 'commander';
import fs           from 'node:fs';
import path         from 'node:path';
import * as url     from 'node:url';
import parseEnvFile from './envParser.js';
import handlers     from "./services/handlers.js";
import readPipe     from "./readPipe.js";
import EnvObject    from './envObject.js';

import log, {setLogDebug}         from './log.js';
import {Options, qualifyingRules} from './components/qualifyingRules.js';
import RuleViolationError         from './errors/RuleViolationError.js';
import escapeAndQuote             from "./escapeAndQuote.js";

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
    .option('--no-json', 'Output as plain text')
    .option('-m, --multiline', 'Allow multiline values')
    .option('-s, --set <value>', 'Update the environment variable in the .env file')
    .option('-q, --quote', 'Quote the value when --set regardless of need')
    .option('-D, --delete', 'Delete the environment variable from the .env file')
    .option('-d, --debug', 'Output extra debugging')
    .showSuggestionAfterError(true)
    .parse(process.argv);

  const cliOptions = program.opts();

  setLogDebug(cliOptions.debug);

  const stdin: string | void = await readPipe().catch((err) => {
    throw new RuleViolationError(`Error reading from stdin: ${err}`);
  });

  const envFilePath: string = cliOptions.file || process.env.DOTENV_FILE || '.env';
  const fullEnvPath: string = path.resolve(envFilePath);
  const keys: string[]      = program.args;
  const set: string         = cliOptions.set;

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
  log.debug('File:', fullEnvPath);

  // Must have a .env file
  if (!fs.existsSync(fullEnvPath)) {
    throw new RuleViolationError(`File not found: ${fullEnvPath}`);
  }

  let options: Options = {
    fullEnvPath:   fullEnvPath,
    envObject:     new EnvObject(),
    json:          (process.argv.includes('--json')),    // commander.js sets json instead of no-json
    noJson:        (process.argv.includes('--no-json')), // commander.js fails to parse this
    multiline:     (cliOptions.multiline !== undefined),
    quote:         (cliOptions.quote !== undefined),
    action:        {
      set:    (setValue !== ''),
      delete: (cliOptions.delete !== undefined),
    },
    singleKey:     (keys.length === 1),
    returnAllKeys: (keys.length === 0),
    targetKeys:    keys,
    setValue:      escapeAndQuote(setValue, (cliOptions.quote !== undefined)),
  };

  // Multiple keys or no keys assume --json
  if ((keys.length > 1 || !keys.length) && !options['noJson']) {
    log.debug('Key count (0 or >1) defaulting to JSON');
    options.json = true;
  }
  log.debug('Options:', options);

  qualifyingRules(options);

  options.envObject = parseEnvFile(envFilePath);

  if (keys.length === 1 && !options['noJson']) {
    log.debug('Single key, and not --no-json, checking for wildcard');

    if (options.targetKeys[0].includes('*')) {
      log.debug('Wildcard found')
      const target: string  = options.targetKeys[0].replace('*', '.*');
      const regex: RegExp = new RegExp('^' + target + '$');
      let i: number       = 0;
      for (const key in options.envObject) {
        if (key.match(regex)) {
          log.debug(`Adding "${key}" to targetKeys`);
          options.targetKeys[i] = key;
          i++;
        }
      }
      options.json = true;
    }
  }

  if (options.json && options.returnAllKeys) {
    log.debug('Outputting entire .env file as JSON');
    log.info(options.envObject.toJsonString(options.multiline));
  } else if (options.action.delete) {
    handlers.deleteKey(options);
  } else if (options.action.set) {
    handlers.setValue(options);
  } else {
    handlers.getValue(options);
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

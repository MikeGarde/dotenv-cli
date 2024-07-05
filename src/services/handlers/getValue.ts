import formatValue from "../../formatValue.js";
import log         from "../../log.js";
import {Options}   from "../../components/qualifyingRules.js";

export default function getValue(options: Options) {
  let result: string = '';

  if (options.targetKeys.length === 0) {
    options.targetKeys = Object.keys(options.envObject);
  }

  for (const key of options.targetKeys) {
    log.debug(`Getting "${key}"`);

    let value = '';

    if (!options.envObject[key]) {
      log.debug(`Environment variable "${key}" not found`);
      process.exitCode = 1;
    } else {
      value = formatValue(options.envObject[key].value, options.multiline);
    }

    value = options.json ? (value ? `"${value}"` : 'null') : value;
    result += options.json ? `"${key}": ${value},` : `${value}\n`;
  }

  // Removes trailing newline or comma
  result = result.slice(0, -1);
  if (options.json) {
    result = `{${result}}`;
  }
  log.info(result);
};

import log         from "../../log.js";
import formatValue from "../../formatValue.js";
import EnvObject   from "../../envObject.js";

export default function getValue(envObject: EnvObject, keys: string[], json: boolean, multiline: boolean) {
  let result: string = '';

  for (const key of keys) {
    log.debug(`Getting "${key}"`);

    let value = '';

    if (!envObject[key]) {
      log.debug(`Environment variable "${key}" not found`);
      process.exitCode = 1;
    } else {
      value = formatValue(envObject[key].value, multiline);
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
};

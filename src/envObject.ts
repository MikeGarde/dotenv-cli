import log from './log.js';

export class EnvValue {
  value: string;
  lineStart: number;
  lineEnd: number;

  constructor(value: string, lineStart: number = -1, lineEnd: number = -1) {
    this.value     = value;
    this.lineStart = lineStart;
    this.lineEnd   = lineEnd;
  }
}

class EnvObject {
  [key: string]: EnvValue | any;

  constructor() {
    return new Proxy(this, {
      set(target, key: PropertyKey, value, receiver) {
        if (typeof key !== 'string') {
          key = 'value';
        }
        if (value instanceof EnvValue) {
          target[key] = value;
          return true;
        }
        if (typeof value === 'object' && value !== null) {
          target[key] = new EnvValue(value.value, value.lineStart, value.lineEnd);
          return true;
        }
        if (typeof value === 'string') {
          if (target[key] instanceof EnvValue) {
            target[key] = value;
          } else {
            target[key] = new EnvValue(value);
          }

        }
        if (typeof target[key] === 'object' && target[key] !== null) {
          target[key].value = value;
        } else {
          // TODO: let's not allow this
          target[key as string] = {
            value:     value,
            lineStart: -1, // You might want to set these values appropriately
            lineEnd:   -1
          };
        }
        return true;
      }
    });
  }

  resolveNestedVariables(): void {
    const variablePattern = /\$\{([a-zA-Z0-9_]+)\}/g;

    for (const key in this) {
      const envValue = this[key] as EnvValue;
      if (envValue instanceof EnvValue) {
        let value = envValue.value;

        if (!value.includes('${')) {
          continue;
        }

        let previousValue;
        do {
          previousValue = value;
          value = value.replace(variablePattern, (match: string, varName: string): string => {
            const nestedEnvValue = this[varName] as EnvValue;
            if (nestedEnvValue instanceof EnvValue) {
              log.debug(`Replacing variable ${varName} with value ${nestedEnvValue.value}`);
              return nestedEnvValue.value;
            }
            log.debug(`Variable ${varName} not found, leaving as is`);
            return match;
          });
        } while (value !== previousValue);

        if (value !== envValue.value) {
          envValue.value = value;
        }
      }
    }
  }

  toObj(): { [key: string]: string } {
    let obj: { [key: string]: string } = {};

    for (const key in this) {
      const keyName: string = key as string;
      const value           = this[keyName].value;

      if (typeof value === 'string') {
        obj[keyName] = value;
      }
    }
    return obj;
  }

  toJsonString(pretty: boolean = false): string {
    if (pretty) {
      return JSON.stringify(this.toObj(), null, 2);
    } else {
      return JSON.stringify(this.toObj());
    }
  }
}

export default EnvObject;

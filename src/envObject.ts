/**
 * Used to create an EnvObject that can be used to store environment variables
 */
class EnvValue {
  value: string;
  lineStart: number;
  lineEnd: number;

  constructor(value: string, lineStart: number = -1, lineEnd: number = -1) {
    this.value = value;
    this.lineStart = lineStart;
    this.lineEnd = lineEnd;
  }
}

/**
 * Used to create an object that can be used to store environment variables
 */
class EnvObject {
  [key: string]: EnvValue | any;

  /**
   * Constructor for the EnvObject class.
   */
  constructor() {
    return new Proxy(this, {
      /**
       * GET trap that is used to get the value of EnvObject
       */
      // get(target, key: PropertyKey, receiver) {
      //   if (typeof key === 'string' && typeof target[key] === 'object' && target[key] !== null) {
      //     console.log('getting', key, target[key]);
      //     return Reflect.get(target[key], 'value', receiver);
      //   } else {
      //     return Reflect.get(target, key, receiver);
      //   }
      // },
      /**
       * SET trap that is used to set the value of EnvObject
       */
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
            value: value,
            lineStart: -1, // You might want to set these values appropriately
            lineEnd: -1
          };
        }
        return true;
      }
    });
  }

  /**
   * Used to convert the EnvObject to an key/value object
   */
  toObj(): { [key: string]: string } {
    let obj: { [key: string]: string } = {};

    for (const key in this) {
      const keyName = key as string;
      const value = this[keyName].value;

      if (typeof value === 'string') {
        obj[keyName] = value;
      }
    }
    return obj;
  }

  /**
   * Used to convert the EnvObject to a JSON string
   */
  toJsonString(): string {
    return JSON.stringify(this.toObj());
  }
}

export default EnvObject;

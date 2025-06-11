import EnvObject, { EnvValue } from '../src/envObject';

describe('EnvObject - resolveNestedVariables', () => {
  test('should resolve a single variable in the string', () => {
    const envObject = new EnvObject();
    envObject['VAR1'] = new EnvValue('Hello');
    envObject['VAR2'] = new EnvValue('${VAR1} World');

    envObject.resolveNestedVariables();

    expect(envObject['VAR2'].value).toBe('Hello World');
  });

  test('should resolve two variables in the string', () => {
    const envObject = new EnvObject();
    envObject['VAR1'] = new EnvValue('Hello');
    envObject['VAR2'] = new EnvValue('World');
    envObject['VAR3'] = new EnvValue('${VAR1} ${VAR2}');

    envObject.resolveNestedVariables();

    expect(envObject['VAR3'].value).toBe('Hello World');
  });

  test('should resolve a previously merged value with another variable', () => {
    const envObject = new EnvObject();
    envObject['VAR1'] = new EnvValue('Hello');
    envObject['VAR2'] = new EnvValue('${VAR1} World');
    envObject['VAR3'] = new EnvValue('${VAR2} & Universe');

    envObject.resolveNestedVariables();

    expect(envObject['VAR3'].value).toBe('Hello World & Universe');
  });
});

import {execSync} from 'child_process';
import path       from 'path';

describe('app.ts', () => {
  const appPath: string = path.resolve(__dirname, '../build/app.js');
  const envPath: string = path.resolve(__dirname, '.env.test');
  const orgHash: string = execSync(`shasum -a 256 ${envPath}`).toString().split(' ')[0];

  test('add a key', () => {
    const setCommand: Buffer = execSync(`node ${appPath} NEW_KEY --set VERY_NEW --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} NEW_KEY --file ${envPath}`);

    expect(setCommand.toString().trim()).toBe('');
    expect(getCommand.toString().trim()).toBe('VERY_NEW');
  });

  test('delete an existing key', () => {
    const delCommand: Buffer = execSync(`node ${appPath} NEW_KEY --delete --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} --file ${envPath}`);
    const allJson: any       = JSON.parse(getCommand.toString().trim());
    const keys: string[]     = Object.keys(allJson);

    expect(delCommand.toString().trim()).toBe('');
    expect(keys).not.toContain('NEW_KEY');
  });

  test('add a new key with a multiline value as a single line', () => {
    const setCommand: Buffer = execSync(`node ${appPath} NEW_ONE --set "This is a\nmultiline value" --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} NEW_ONE --file ${envPath}`);

    expect(setCommand.toString().trim()).toBe('');
    // Note: We're escaping the newline character in the string
    expect(getCommand.toString().trim()).toBe('This is a\\nmultiline value');
  });

  test('add a new key with a multiline value', () => {
    const setCommand: Buffer = execSync(`node ${appPath} NEW_TWO --set "This is a\nmultiline value" --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} NEW_TWO --file ${envPath}`);

    expect(setCommand.toString().trim()).toBe('');
    expect(getCommand.toString().trim()).toBe('This is a\\nmultiline value');
  });

  test('update an existing key without disturbing key/values below it', () => {
    // This also tests that new keys are added to the end of the .env file
    const setCommand: Buffer = execSync(`node ${appPath} NEW_ONE --set "Single line value" --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} --file ${envPath}`);
    const allJson: any       = JSON.parse(getCommand.toString().trim());
    const keys: string[]     = Object.keys(allJson);
    const lastKey: string    = keys[keys.length - 1];
    const lastValue: string  = allJson[lastKey];

    expect(setCommand.toString().trim()).toBe('');
    expect(allJson['NEW_ONE']).toBe('Single line value');
    expect(lastKey).toBe('NEW_TWO');
    expect(lastValue).toBe('This is a\nmultiline value');
  });

  test('update an existing key with a stdin value', () => {
    const setCommand: Buffer = execSync(`echo "New stdin value" | node ${appPath} NEW_TWO --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} NEW_TWO --file ${envPath}`);

    expect(setCommand.toString().trim()).toBe('');
    expect(getCommand.toString().trim()).toBe('New stdin value');
  });

  test('remove all new test keys', () => {
    const delOne: Buffer     = execSync(`node ${appPath} NEW_ONE --delete --file ${envPath}`);
    const delTwo: Buffer     = execSync(`node ${appPath} NEW_TWO --delete --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} --file ${envPath}`);
    const allJson: any       = JSON.parse(getCommand.toString().trim());
    const keys: string[]     = Object.keys(allJson);

    expect(delOne.toString().trim()).toBe('');
    expect(delTwo.toString().trim()).toBe('');
    expect(keys).not.toContain('NEW_ONE');
    expect(keys).not.toContain('NEW_TWO');
  });

  test('after above .env file is unchanged', () => {
    const hash: string = execSync(`shasum -a 256 ${envPath}`).toString().split(' ')[0];
    expect(hash).toBe(orgHash);
  });

});

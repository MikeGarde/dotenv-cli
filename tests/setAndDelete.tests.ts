import {execSync} from 'child_process';
import path       from 'path';

describe('app.ts', () => {
  const appPath: string = path.resolve(__dirname, '../build/app.js');
  const envPath: string = path.resolve(__dirname, '.env.test');
  const shaHash: string = execSync(`shasum -a 256 ${envPath}`).toString().split(' ')[0];

  it('should insert a new key and value to the end of the .env file', () => {
    const setCommand: Buffer = execSync(`node ${appPath} NEW_KEY --set NEW_VALUE --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} --file ${envPath}`);
    const allJson: any       = JSON.parse(getCommand.toString().trim());
    const keys: string[]     = Object.keys(allJson);
    const lastKey: string    = keys[keys.length - 1];
    const lastValue: string  = allJson[lastKey];

    expect(setCommand.toString().trim()).toBe('');
    expect(allJson['NEW_KEY']).toBe('NEW_VALUE');
    expect(lastKey).toBe('NEW_KEY');
    expect(lastValue).toBe('NEW_VALUE');
  });

  it('should update an existing key with a new value', () => {
    const setCommand: Buffer    = execSync(`node ${appPath} NEW_KEY --set VERY_NEW --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} NAME --file ${envPath}`);

    expect(setCommand.toString().trim()).toBe('');
    expect(getCommand.toString().trim()).toBe('VERY_NEW');
  });

  it('should delete an existing key', () => {
    const delCommand: Buffer    = execSync(`node ${appPath} NAME --delete --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} NAME --file ${envPath}`);

    expect(delCommand.toString().trim()).toBe('');
    expect(getCommand.toString().trim()).toBe('');
  });

  it('should add a new key with a multiline value as a single line', () => {
    const setCommand: Buffer    = execSync(`node ${appPath} NEW_ONE --set "This is a\nmultiline value" --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} NEW_ONE --file ${envPath}`);

    expect(setCommand.toString().trim()).toBe('');
    expect(getCommand.toString().trim()).toBe('This is a multiline value');
  });

  it('should add a new key with a multiline value', () => {
    const setCommand: Buffer    = execSync(`node ${appPath} NEW_TWO --set "This is a\nmultiline value" --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} NEW_TWO --file ${envPath}`);

    expect(setCommand.toString().trim()).toBe('');
    expect(getCommand.toString().trim()).toBe('This is a\nmultiline value');
  });

  it('should update an existing key without disturbing key/values below it', () => {
    const setCommand: Buffer    = execSync(`node ${appPath} NEW_ONE --set "New double quotes" --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} NEW_ONE --file ${envPath}`);
    const allJson: any       = JSON.parse(getCommand.toString().trim());
    const keys: string[]     = Object.keys(allJson);
    const lastKey: string    = keys[keys.length - 1];
    const lastValue: string  = allJson[lastKey];

    expect(setCommand.toString().trim()).toBe('');
    expect(allJson['NEW_ONE']).toBe('New double quotes');
    expect(lastKey).toBe('NEW_TWO');
    expect(lastValue).toBe('This is a\nmultiline value');
  });

  it('should update an existing key with a stdin value', () => {
    const result: Buffer = execSync(`echo "New stdin value" | node ${appPath} NEW_TWO --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} NEW_TWO --file ${envPath}`);

    expect(result.toString().trim()).toBe('');
    expect(getCommand.toString().trim()).toBe('New stdin value');
  });

  it('should remove all new test keys', () => {
    const delOne: Buffer = execSync(`node ${appPath} NEW_ONE --delete --file ${envPath}`);
    const delTwo: Buffer = execSync(`node ${appPath} NEW_TWO --delete --file ${envPath}`);
    const getCommand: Buffer = execSync(`node ${appPath} --file ${envPath}`);
    const allJson: any       = JSON.parse(getCommand.toString().trim());
    const keys: string[]     = Object.keys(allJson);

    expect(delOne.toString().trim()).toBe('');
    expect(delTwo.toString().trim()).toBe('');
    expect(keys).not.toContain('NEW_ONE');
    expect(keys).not.toContain('NEW_TWO');
  });

});

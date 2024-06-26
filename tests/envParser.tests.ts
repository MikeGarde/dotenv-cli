import parseEnvFile, { EnvObject } from '../src/envParser';

describe('envParser', () => {
  it('should parse', () => {
    const envObject: EnvObject = parseEnvFile('tests/.env.test');
    const envCount: number     = Object.keys(envObject).length;

    expect(envCount).toBe(7);
  });
});

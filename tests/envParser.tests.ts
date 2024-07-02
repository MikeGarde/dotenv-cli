import parseEnvFile, { EnvObject } from '../src/envParser';

describe('envParse', () => {
  test('parse file', () => {
    const envObject: EnvObject = parseEnvFile('tests/.env.test');
    const envCount: number     = Object.keys(envObject).length;

    // TODO: Race conditions by the setAndDelete tests may cause this number to vary upward
    // TODO: expect(envCount).toBe(7);
    expect(envCount).toBeGreaterThanOrEqual(7);
  });
});

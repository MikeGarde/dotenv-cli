export default class EnvParseError extends Error {
  constructor(line: number, message: string) {
    super(`Error parsing .env file at line ${line}: ${message}`);
  }
}

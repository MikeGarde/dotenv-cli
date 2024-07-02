export default class RuleViolationError extends Error {
  constructor(message: string) {
    super(message);
    Object.setPrototypeOf(this, RuleViolationError.prototype);
  }
}

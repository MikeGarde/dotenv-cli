/**
 * RuleViolation
 */
export default class RuleViolation extends Error {
  constructor(message: string) {
    super(message);
    Object.setPrototypeOf(this, RuleViolation.prototype);
  }
}

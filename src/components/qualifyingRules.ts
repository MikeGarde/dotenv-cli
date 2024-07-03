import fs                 from "node:fs";
import RuleViolationError from "../errors/RuleViolationError.js";
import EnvObject          from "../envObject.js";

export interface Options {
  fullEnvPath: string;
  envObject: EnvObject;
  json: boolean;
  multiline: boolean;
  quote: boolean;
  action: {
    set: boolean;
    delete: boolean;
  }
  singleKey: boolean;
  returnAllKeys: boolean;
  targetKeys: string[];
  setValue: string | null;
}

export function qualifyingRules(settings: Options) {
  // - cannot have both --json and --set
  if (settings.json && settings.action.set) {
    throw new RuleViolationError('Cannot use --json and --set together');
  }
  // - must have a key if using --set
  if (settings.action.set && !settings.singleKey) {
    throw new RuleViolationError('Must specify a single key when using --set');
  }
  // - cannot use --delete with any other options
  if (settings.action.delete && (settings.action.set || settings.json || settings.multiline)) {
    throw new RuleViolationError('Cannot use --delete with any other options');
  }
  // - must have a key if using --delete
  if (settings.action.delete && !settings.singleKey) {
    throw new RuleViolationError('Must specify a single key when using --delete');
  }
}

import loglevel from 'loglevel';

const log = loglevel.getLogger('dotenv');

// trace debug info warn error
log.setLevel(loglevel.levels.INFO);

export function setLogDebug(debug: boolean) {
  if (debug) {
    log.setLevel(loglevel.levels.DEBUG);
  } else {
    log.setLevel(loglevel.levels.INFO);
  }
}

export default log;

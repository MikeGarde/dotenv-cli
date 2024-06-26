import loglevel from 'loglevel';

const log = loglevel.getLogger('dotenv');

// Set the default level
// trace debug info warn error
log.setLevel(loglevel.levels.INFO);

// Function to set level based on command line options
export function setLogDebug(debug: boolean) {
  if (debug) {
    log.setLevel(loglevel.levels.DEBUG);
  } else {
    log.setLevel(loglevel.levels.INFO);
  }
}

export default log;

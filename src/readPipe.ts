import {Interface, createInterface}       from 'node:readline';
import {stdin as input, stdout as output} from 'node:process';

/**
 * Read from the pipe and return the data as a string
 */
const readPipe = (): Promise<string> => {
  return new Promise((resolve, reject) => {
    // If the stdin is a TTY device aka no pipe, resolve the promise with an empty string
    if (input.isTTY) {
      resolve('');
      return;
    }

    const rl: Interface = createInterface({input, output});

    let inputData: string = '';

    rl.on('line', (input) => {
      inputData += input + '\n';
    });

    rl.on('close', () => {
      inputData = inputData.trim();
      resolve(inputData);
    });

    rl.on('error', (err) => {
      reject(err);
    });
  });
}

export default readPipe;

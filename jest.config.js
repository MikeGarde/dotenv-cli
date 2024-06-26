/** @type {import('ts-jest').JestConfigWithTsJest} */
export default {
  preset          : 'ts-jest',
  testEnvironment : 'node',
  testMatch       : ['<rootDir>/tests/**/*.tests.ts'],
  moduleNameMapper: {
    '(.+)\\.js': '$1', // Remove the .js extension
  },
};

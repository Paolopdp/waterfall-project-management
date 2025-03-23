import "@testing-library/jest-dom";
import { type TestingLibraryMatchers } from "@testing-library/jest-dom/matchers";

declare global {
  namespace jest {
    interface Matchers<R = void>
      extends TestingLibraryMatchers<typeof expect.stringContaining, R> {}
  }
}

// Suppress React 18 console warnings in test output
const originalError = console.error;
beforeAll(() => {
  console.error = (...args: any[]) => {
    if (/Warning: ReactDOM.render is no longer supported/.test(args[0])) {
      return;
    }
    originalError.call(console, ...args);
  };
});

afterAll(() => {
  console.error = originalError;
});

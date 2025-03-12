import { useEffect, useLayoutEffect } from 'react';

/**
 * A custom hook that uses useLayoutEffect on the client-side and
 * falls back to useEffect during server-side rendering.
 * 
 * This avoids the React warning:
 * "useLayoutEffect does nothing on the server, because its effect cannot
 * be encoded into the server renderer's output format."
 */
const useIsomorphicLayoutEffect = typeof window !== 'undefined' 
  ? useLayoutEffect 
  : useEffect;

export default useIsomorphicLayoutEffect;


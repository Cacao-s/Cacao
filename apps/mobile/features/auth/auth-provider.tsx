import { createContext, useCallback, useContext, useMemo, useState, type ReactNode } from 'react';
import { login as loginRequest, type LoginResponse } from '../../services/api';

export type AuthStatus = 'signedOut' | 'signingIn' | 'authenticated' | 'error';

export type AuthContextValue = {
  token: string | null;
  user: LoginResponse['user'] | null;
  status: AuthStatus;
  error: string | null;
  login: (username: string, password: string) => Promise<void>;
  logout: () => void;
};

const AuthContext = createContext<AuthContextValue | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [token, setToken] = useState<string | null>(null);
  const [user, setUser] = useState<LoginResponse['user'] | null>(null);
  const [status, setStatus] = useState<AuthStatus>('signedOut');
  const [error, setError] = useState<string | null>(null);

  const login = useCallback(async (username: string, password: string) => {
    setStatus('signingIn');
    setError(null);

    try {
      const response = await loginRequest(username, password);
      setToken(response.token);
      setUser(response.user);
      setStatus('authenticated');
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Unable to sign in';
      setError(message);
      setStatus('error');
      setToken(null);
      setUser(null);
      throw err;
    }
  }, []);

  const logout = useCallback(() => {
    setToken(null);
    setUser(null);
    setStatus('signedOut');
    setError(null);
  }, []);

  const value = useMemo<AuthContextValue>(
    () => ({ token, user, status, error, login, logout }),
    [error, login, logout, status, token, user],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return ctx;
}
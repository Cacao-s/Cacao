const API_BASE_URL = process.env.EXPO_PUBLIC_API_URL ?? 'http://localhost:8080';

type RequestOptions = {
  method?: string;
  headers?: Record<string, string>;
  body?: Record<string, unknown> | string;
};

async function request<T>(path: string, options: RequestOptions = {}): Promise<T> {
  const url = `${API_BASE_URL}${path}`;
  const { body, headers, method = 'GET' } = options;

  const init: RequestInit = {
    method,
    headers: {
      'Content-Type': 'application/json',
      ...(headers ?? {}),
    },
    body: typeof body === 'string' ? body : body ? JSON.stringify(body) : undefined,
  };

  const response = await fetch(url, init);
  const data = await response.json().catch(() => null);

  if (!response.ok) {
    const errorMessage = (data as any)?.error ?? 'Request failed';
    throw new Error(errorMessage);
  }

  return data as T;
}

export type LoginResponse = {
  token: string;
  expiresAt: string;
  user: {
    username: string;
    displayName: string;
    roles: string[];
  };
};

export async function login(username: string, password: string) {
  return request<LoginResponse>('/api/v1/auth/login', {
    method: 'POST',
    body: { username, password },
  });
}
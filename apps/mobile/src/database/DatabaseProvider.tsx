import React from 'react';
import { DatabaseProvider as WatermelonProvider } from '@nozbe/watermelondb/react';
import { database } from './index';

interface DatabaseProviderProps {
  children: React.ReactNode;
}

/**
 * Provides WatermelonDB database context to the app
 * Wrap your app root with this provider to enable database access
 */
export function DatabaseProvider({ children }: DatabaseProviderProps) {
  return <WatermelonProvider database={database}>{children}</WatermelonProvider>;
}

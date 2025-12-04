import { Database } from '@nozbe/watermelondb';
import SQLiteAdapter from '@nozbe/watermelondb/adapters/sqlite';

import { schema } from './schema';
import {
  User,
  Family,
  FamilyMember,
  Wallet,
  Allowance,
  Request,
  Transaction,
  Notification,
  SyncQueue,
  AuditLog,
} from '../models';

// Create SQLite adapter
const adapter = new SQLiteAdapter({
  schema,
  // Optional: Define migrations for schema changes
  // migrations,
  // Optional: Enable JSI for better performance (requires New Architecture)
  // jsi: true,
  // Optional: Database name
  dbName: 'cacao',
});

// Create Database instance
export const database = new Database({
  adapter,
  modelClasses: [
    User,
    Family,
    FamilyMember,
    Wallet,
    Allowance,
    Request,
    Transaction,
    Notification,
    SyncQueue,
    AuditLog,
  ],
});

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

// Mobile (iOS/Android): SQLiteAdapter with full model classes
const adapter = new SQLiteAdapter({
  schema,
  dbName: 'cacao',
});

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

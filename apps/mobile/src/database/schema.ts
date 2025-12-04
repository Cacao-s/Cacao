import { appSchema, tableSchema } from '@nozbe/watermelondb';

export const schema = appSchema({
  version: 1,
  tables: [
    tableSchema({
      name: 'users',
      columns: [
        { name: 'email', type: 'string', isIndexed: true },
        { name: 'password_hash', type: 'string', isOptional: true },
        { name: 'google_sub', type: 'string', isOptional: true, isIndexed: true },
        { name: 'display_name', type: 'string' },
        { name: 'locale', type: 'string' },
        { name: 'theme', type: 'string' },
        { name: 'role', type: 'string' }, // 'giver' | 'baby' | 'admin'
        { name: 'status', type: 'string' }, // 'active' | 'invited' | 'disabled'
        { name: 'created_at', type: 'number' },
        { name: 'updated_at', type: 'number' },
      ],
    }),
    tableSchema({
      name: 'families',
      columns: [
        { name: 'name', type: 'string' },
        { name: 'currency', type: 'string' },
        { name: 'timezone', type: 'string' },
        { name: 'created_by', type: 'string', isIndexed: true }, // user_id as foreign key
        { name: 'created_at', type: 'number' },
        { name: 'updated_at', type: 'number' },
      ],
    }),
    tableSchema({
      name: 'family_members',
      columns: [
        { name: 'family_id', type: 'string', isIndexed: true },
        { name: 'user_id', type: 'string', isIndexed: true },
        { name: 'family_role', type: 'string' }, // 'giver' | 'baby' | 'viewer'
        { name: 'invited_by', type: 'string', isOptional: true },
        { name: 'status', type: 'string' }, // 'active' | 'pending' | 'removed'
        { name: 'joined_at', type: 'number', isOptional: true },
        { name: 'created_at', type: 'number' },
        { name: 'updated_at', type: 'number' },
      ],
    }),
    tableSchema({
      name: 'wallets',
      columns: [
        { name: 'family_id', type: 'string', isIndexed: true },
        { name: 'name', type: 'string' },
        { name: 'type', type: 'string' }, // 'cash' | 'bank' | 'card' | 'virtual'
        { name: 'balance_cents', type: 'number' },
        { name: 'currency', type: 'string' },
        { name: 'warning_threshold_cents', type: 'number' },
        { name: 'status', type: 'string' }, // 'active' | 'archived'
        { name: 'created_at', type: 'number' },
        { name: 'updated_at', type: 'number' },
      ],
    }),
    tableSchema({
      name: 'allowances',
      columns: [
        { name: 'family_id', type: 'string', isIndexed: true },
        { name: 'giver_member_id', type: 'string' },
        { name: 'receiver_member_id', type: 'string' },
        { name: 'wallet_id', type: 'string', isIndexed: true },
        { name: 'amount_cents', type: 'number' },
        { name: 'frequency', type: 'string' }, // 'daily' | 'weekly' | 'biweekly' | 'monthly' | 'custom'
        { name: 'interval_count', type: 'number' },
        { name: 'next_run_at', type: 'number', isOptional: true },
        { name: 'last_run_at', type: 'number', isOptional: true },
        { name: 'status', type: 'string' }, // 'active' | 'paused' | 'archived'
        { name: 'notes', type: 'string', isOptional: true },
        { name: 'created_at', type: 'number' },
        { name: 'updated_at', type: 'number' },
      ],
    }),
    tableSchema({
      name: 'requests',
      columns: [
        { name: 'family_id', type: 'string', isIndexed: true },
        { name: 'requester_member_id', type: 'string' },
        { name: 'wallet_id', type: 'string', isIndexed: true },
        { name: 'amount_cents', type: 'number' },
        { name: 'category', type: 'string', isOptional: true },
        { name: 'notes', type: 'string', isOptional: true },
        { name: 'attachment_url', type: 'string', isOptional: true },
        { name: 'status', type: 'string', isIndexed: true }, // 'draft' | 'pending' | 'approved' | 'rejected' | 'cancelled'
        { name: 'decision_by_member_id', type: 'string', isOptional: true },
        { name: 'decision_at', type: 'number', isOptional: true },
        { name: 'rejection_reason', type: 'string', isOptional: true },
        { name: 'created_at', type: 'number' },
        { name: 'updated_at', type: 'number' },
      ],
    }),
    tableSchema({
      name: 'transactions',
      columns: [
        { name: 'family_id', type: 'string', isIndexed: true },
        { name: 'wallet_id', type: 'string', isIndexed: true },
        { name: 'type', type: 'string' }, // 'credit' | 'debit'
        { name: 'amount_cents', type: 'number' },
        { name: 'source_type', type: 'string' }, // 'allowance' | 'request' | 'manual' | 'adjustment'
        { name: 'source_id', type: 'number', isOptional: true },
        { name: 'category', type: 'string', isOptional: true },
        { name: 'occurred_at', type: 'number' },
        { name: 'notes', type: 'string', isOptional: true },
        { name: 'created_at', type: 'number' },
      ],
    }),
    tableSchema({
      name: 'notifications',
      columns: [
        { name: 'user_id', type: 'string', isIndexed: true },
        { name: 'event_type', type: 'string' },
        { name: 'payload', type: 'string', isOptional: true }, // JSON string
        { name: 'delivery_status', type: 'string', isIndexed: true }, // 'pending' | 'sent' | 'failed' | 'read'
        { name: 'read_at', type: 'number', isOptional: true },
        { name: 'created_at', type: 'number' },
      ],
    }),
    tableSchema({
      name: 'sync_queue',
      columns: [
        { name: 'device_id', type: 'string' },
        { name: 'user_id', type: 'string' },
        { name: 'operation_type', type: 'string' },
        { name: 'payload', type: 'string' }, // JSON string
        { name: 'temp_id', type: 'string' },
        { name: 'status', type: 'string' }, // 'pending' | 'synced' | 'failed'
        { name: 'retries', type: 'number' },
        { name: 'last_error', type: 'string', isOptional: true },
        { name: 'created_at', type: 'number' },
        { name: 'updated_at', type: 'number' },
      ],
    }),
    tableSchema({
      name: 'audit_logs',
      columns: [
        { name: 'actor_id', type: 'string', isOptional: true },
        { name: 'family_id', type: 'string', isOptional: true, isIndexed: true },
        { name: 'action', type: 'string', isIndexed: true },
        { name: 'resource_type', type: 'string' },
        { name: 'resource_id', type: 'string', isOptional: true },
        { name: 'metadata', type: 'string', isOptional: true }, // JSON string
        { name: 'created_at', type: 'number' },
      ],
    }),
  ],
});

import { Model } from '@nozbe/watermelondb';
import { field, date, readonly, relation } from '@nozbe/watermelondb/decorators';
import type { Associations } from '@nozbe/watermelondb/Model';
import type User from './User';

export type SyncStatus = 'pending' | 'synced' | 'failed';

/**
 * SyncQueue model for offline-first data synchronization
 * Tracks operations that need to be synced with the backend
 */
export default class SyncQueue extends Model {
  static table = 'sync_queue';

  static associations: Associations = {
    users: { type: 'belongs_to', key: 'user_id' },
  };

  @field('device_id') deviceId!: string;
  @field('user_id') userId!: string;
  @field('operation_type') operationType!: string; // 'create', 'update', 'delete'
  @field('payload') payload!: string; // JSON string containing the operation data
  @field('temp_id') tempId!: string; // Temporary ID for optimistic updates
  @field('status') status!: SyncStatus;
  @field('retries') retries!: number;
  @field('last_error') lastError?: string;

  @readonly @date('created_at') createdAt!: Date;
  @readonly @date('updated_at') updatedAt!: Date;

  @relation('users', 'user_id') user!: User;

  // Helper method to parse JSON payload
  get parsedPayload(): any {
    try {
      return JSON.parse(this.payload);
    } catch {
      return null;
    }
  }
}

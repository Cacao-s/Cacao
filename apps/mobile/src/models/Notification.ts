import { Model } from '@nozbe/watermelondb';
import { field, date, readonly, relation } from '@nozbe/watermelondb/decorators';
import type { Associations } from '@nozbe/watermelondb/Model';
import type User from './User';

export type NotificationDeliveryStatus = 'pending' | 'sent' | 'failed' | 'read';

export default class Notification extends Model {
  static table = 'notifications';

  static associations: Associations = {
    users: { type: 'belongs_to', key: 'user_id' },
  };

  @field('user_id') userId!: string;
  @field('event_type') eventType!: string;
  @field('payload') payload?: string; // JSON string
  @field('delivery_status') deliveryStatus!: NotificationDeliveryStatus;
  @date('read_at') readAt?: Date;

  @readonly @date('created_at') createdAt!: Date;

  @relation('users', 'user_id') user!: User;

  // Helper method to parse JSON payload
  get parsedPayload(): any {
    try {
      return this.payload ? JSON.parse(this.payload) : null;
    } catch {
      return null;
    }
  }
}

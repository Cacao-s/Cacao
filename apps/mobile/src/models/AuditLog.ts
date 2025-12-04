import { Model } from '@nozbe/watermelondb';
import { field, date, readonly, relation } from '@nozbe/watermelondb/decorators';
import type { Associations } from '@nozbe/watermelondb/Model';
import type User from './User';
import type Family from './Family';

export default class AuditLog extends Model {
  static table = 'audit_logs';

  static associations: Associations = {
    users: { type: 'belongs_to', key: 'actor_id' },
    families: { type: 'belongs_to', key: 'family_id' },
  };

  @field('actor_id') actorId?: string;
  @field('family_id') familyId?: string;
  @field('action') action!: string;
  @field('resource_type') resourceType!: string;
  @field('resource_id') resourceId?: string;
  @field('metadata') metadata?: string; // JSON string

  @readonly @date('created_at') createdAt!: Date;

  @relation('users', 'actor_id') actor?: User;
  @relation('families', 'family_id') family?: Family;

  // Helper method to parse JSON metadata
  get parsedMetadata(): any {
    try {
      return this.metadata ? JSON.parse(this.metadata) : null;
    } catch {
      return null;
    }
  }
}

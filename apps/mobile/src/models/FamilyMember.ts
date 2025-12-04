import { Model } from '@nozbe/watermelondb';
import { field, date, readonly, relation } from '@nozbe/watermelondb/decorators';
import type { Associations } from '@nozbe/watermelondb/Model';
import type Family from './Family';
import type User from './User';

export type FamilyRole = 'giver' | 'baby' | 'viewer';
export type MemberStatus = 'active' | 'pending' | 'removed';

export default class FamilyMember extends Model {
  static table = 'family_members';

  static associations: Associations = {
    families: { type: 'belongs_to', key: 'family_id' },
    users: { type: 'belongs_to', key: 'user_id' },
  };

  @field('family_id') familyId!: string;
  @field('user_id') userId!: string;
  @field('family_role') familyRole!: FamilyRole;
  @field('invited_by') invitedBy?: string;
  @field('status') status!: MemberStatus;
  @date('joined_at') joinedAt?: Date;

  @readonly @date('created_at') createdAt!: Date;
  @readonly @date('updated_at') updatedAt!: Date;

  @relation('families', 'family_id') family!: Family;
  @relation('users', 'user_id') user!: User;
}

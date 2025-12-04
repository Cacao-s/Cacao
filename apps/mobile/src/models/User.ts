import { Model } from '@nozbe/watermelondb';
import { field, date, readonly, children } from '@nozbe/watermelondb/decorators';
import type { Associations } from '@nozbe/watermelondb/Model';

export type UserRole = 'giver' | 'baby' | 'admin';
export type UserStatus = 'active' | 'invited' | 'disabled';

export default class User extends Model {
  static table = 'users';

  static associations: Associations = {
    families: { type: 'has_many', foreignKey: 'created_by' },
    family_members: { type: 'has_many', foreignKey: 'user_id' },
    notifications: { type: 'has_many', foreignKey: 'user_id' },
  };

  @field('email') email!: string;
  @field('password_hash') passwordHash?: string;
  @field('google_sub') googleSub?: string;
  @field('display_name') displayName!: string;
  @field('locale') locale!: string;
  @field('theme') theme!: string;
  @field('role') role!: UserRole;
  @field('status') status!: UserStatus;

  @readonly @date('created_at') createdAt!: Date;
  @readonly @date('updated_at') updatedAt!: Date;

  @children('families') families!: any;
  @children('family_members') familyMembers!: any;
  @children('notifications') notifications!: any;
}

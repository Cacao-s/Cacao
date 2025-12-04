import { Model } from '@nozbe/watermelondb';
import { field, date, readonly, relation, children } from '@nozbe/watermelondb/decorators';
import type { Associations } from '@nozbe/watermelondb/Model';
import type User from './User';

export default class Family extends Model {
  static table = 'families';

  static associations: Associations = {
    users: { type: 'belongs_to', key: 'created_by' },
    family_members: { type: 'has_many', foreignKey: 'family_id' },
    wallets: { type: 'has_many', foreignKey: 'family_id' },
    allowances: { type: 'has_many', foreignKey: 'family_id' },
    requests: { type: 'has_many', foreignKey: 'family_id' },
    transactions: { type: 'has_many', foreignKey: 'family_id' },
  };

  @field('name') name!: string;
  @field('currency') currency!: string;
  @field('timezone') timezone!: string;
  @field('created_by') createdBy!: string;

  @readonly @date('created_at') createdAt!: Date;
  @readonly @date('updated_at') updatedAt!: Date;

  @relation('users', 'created_by') creator!: User;
  @children('family_members') members!: any;
  @children('wallets') wallets!: any;
  @children('allowances') allowances!: any;
  @children('requests') requests!: any;
  @children('transactions') transactions!: any;
}

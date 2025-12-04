import { Model } from '@nozbe/watermelondb';
import { field, date, readonly, relation, children } from '@nozbe/watermelondb/decorators';
import type { Associations } from '@nozbe/watermelondb/Model';
import type Family from './Family';

export type WalletType = 'cash' | 'bank' | 'card' | 'virtual';
export type WalletStatus = 'active' | 'archived';

export default class Wallet extends Model {
  static table = 'wallets';

  static associations: Associations = {
    families: { type: 'belongs_to', key: 'family_id' },
    allowances: { type: 'has_many', foreignKey: 'wallet_id' },
    requests: { type: 'has_many', foreignKey: 'wallet_id' },
    transactions: { type: 'has_many', foreignKey: 'wallet_id' },
  };

  @field('family_id') familyId!: string;
  @field('name') name!: string;
  @field('type') type!: WalletType;
  @field('balance_cents') balanceCents!: number;
  @field('currency') currency!: string;
  @field('warning_threshold_cents') warningThresholdCents!: number;
  @field('status') status!: WalletStatus;

  @readonly @date('created_at') createdAt!: Date;
  @readonly @date('updated_at') updatedAt!: Date;

  @relation('families', 'family_id') family!: Family;
  @children('allowances') allowances!: any;
  @children('requests') requests!: any;
  @children('transactions') transactions!: any;
}

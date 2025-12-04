import { Model } from '@nozbe/watermelondb';
import { field, date, readonly, relation } from '@nozbe/watermelondb/decorators';
import type { Associations } from '@nozbe/watermelondb/Model';
import type Family from './Family';
import type FamilyMember from './FamilyMember';
import type Wallet from './Wallet';

export type AllowanceFrequency = 'daily' | 'weekly' | 'biweekly' | 'monthly' | 'custom';
export type AllowanceStatus = 'active' | 'paused' | 'archived';

export default class Allowance extends Model {
  static table = 'allowances';

  static associations: Associations = {
    families: { type: 'belongs_to', key: 'family_id' },
    wallets: { type: 'belongs_to', key: 'wallet_id' },
  };

  @field('family_id') familyId!: string;
  @field('giver_member_id') giverMemberId!: string;
  @field('receiver_member_id') receiverMemberId!: string;
  @field('wallet_id') walletId!: string;
  @field('amount_cents') amountCents!: number;
  @field('frequency') frequency!: AllowanceFrequency;
  @field('interval_count') intervalCount!: number;
  @date('next_run_at') nextRunAt?: Date;
  @date('last_run_at') lastRunAt?: Date;
  @field('status') status!: AllowanceStatus;
  @field('notes') notes?: string;

  @readonly @date('created_at') createdAt!: Date;
  @readonly @date('updated_at') updatedAt!: Date;

  @relation('families', 'family_id') family!: Family;
  @relation('wallets', 'wallet_id') wallet!: Wallet;
}

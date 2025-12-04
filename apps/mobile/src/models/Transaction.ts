import { Model } from '@nozbe/watermelondb';
import { field, date, readonly, relation } from '@nozbe/watermelondb/decorators';
import type { Associations } from '@nozbe/watermelondb/Model';
import type Family from './Family';
import type Wallet from './Wallet';

export type TransactionType = 'credit' | 'debit';
export type TransactionSourceType = 'allowance' | 'request' | 'manual' | 'adjustment';

export default class Transaction extends Model {
  static table = 'transactions';

  static associations: Associations = {
    families: { type: 'belongs_to', key: 'family_id' },
    wallets: { type: 'belongs_to', key: 'wallet_id' },
  };

  @field('family_id') familyId!: string;
  @field('wallet_id') walletId!: string;
  @field('type') type!: TransactionType;
  @field('amount_cents') amountCents!: number;
  @field('source_type') sourceType!: TransactionSourceType;
  @field('source_id') sourceId?: number;
  @field('category') category?: string;
  @date('occurred_at') occurredAt!: Date;
  @field('notes') notes?: string;

  @readonly @date('created_at') createdAt!: Date;

  @relation('families', 'family_id') family!: Family;
  @relation('wallets', 'wallet_id') wallet!: Wallet;
}

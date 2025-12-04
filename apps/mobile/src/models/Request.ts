import { Model } from '@nozbe/watermelondb';
import { field, date, readonly, relation } from '@nozbe/watermelondb/decorators';
import type { Associations } from '@nozbe/watermelondb/Model';
import type Family from './Family';
import type Wallet from './Wallet';

export type RequestStatus = 'draft' | 'pending' | 'approved' | 'rejected' | 'cancelled';

export default class Request extends Model {
  static table = 'requests';

  static associations: Associations = {
    families: { type: 'belongs_to', key: 'family_id' },
    wallets: { type: 'belongs_to', key: 'wallet_id' },
  };

  @field('family_id') familyId!: string;
  @field('requester_member_id') requesterMemberId!: string;
  @field('wallet_id') walletId!: string;
  @field('amount_cents') amountCents!: number;
  @field('category') category?: string;
  @field('notes') notes?: string;
  @field('attachment_url') attachmentUrl?: string;
  @field('status') status!: RequestStatus;
  @field('decision_by_member_id') decisionByMemberId?: string;
  @date('decision_at') decisionAt?: Date;
  @field('rejection_reason') rejectionReason?: string;

  @readonly @date('created_at') createdAt!: Date;
  @readonly @date('updated_at') updatedAt!: Date;

  @relation('families', 'family_id') family!: Family;
  @relation('wallets', 'wallet_id') wallet!: Wallet;
}

export interface Profile {
  id: number;
  uuid: string;
  display_name: string;
  role: string;
  locale: string;
  theme: string;
  pin_hash: string | null;
  sync_version: number;
  last_synced_at: string | null;
  is_deleted: number;
  created_at: string;
  updated_at: string;
}

export interface Family {
  id: number;
  uuid: string;
  name: string;
  currency: string;
  timezone: string;
  created_by_device: string;
  sync_version: number;
  last_synced_at: string | null;
  is_deleted: number;
  created_at: string;
  updated_at: string;
}

export interface FamilyMember {
  id: number;
  uuid: string;
  family_id: number;
  profile_uuid: string;
  family_role: string;
  status: string;
  joined_at: string | null;
  sync_version: number;
  last_synced_at: string | null;
  is_deleted: number;
  created_at: string;
  updated_at: string;
}

export interface Wallet {
  id: number;
  uuid: string;
  family_id: number;
  name: string;
  type: string;
  balance_cents: number;
  currency: string;
  warning_threshold_cents: number;
  status: string;
  sync_version: number;
  last_synced_at: string | null;
  is_deleted: number;
  created_at: string;
  updated_at: string;
}

export interface Request {
  id: number;
  uuid: string;
  family_id: number;
  requester_member_id: number;
  wallet_id: number;
  amount_cents: number;
  category: string | null;
  notes: string | null;
  attachment_url: string | null;
  status: string;
  decision_by_member_id: number | null;
  decision_at: string | null;
  rejection_reason: string | null;
  sync_version: number;
  last_synced_at: string | null;
  is_deleted: number;
  created_at: string;
  updated_at: string;
}

export interface Transaction {
  id: number;
  uuid: string;
  family_id: number;
  wallet_id: number;
  type: string;
  amount_cents: number;
  source_type: string;
  source_id: number | null;
  category: string | null;
  occurred_at: string;
  notes: string | null;
  sync_version: number;
  last_synced_at: string | null;
  is_deleted: number;
  created_at: string;
}

export type RequestStatus = "draft" | "pending" | "approved" | "rejected" | "cancelled";

export type RequestCategory =
  | "food"
  | "transport"
  | "education"
  | "entertainment"
  | "clothing"
  | "health"
  | "other";

export interface RequestSummary {
  pending: number;
  approved: number;
  rejected: number;
}

export interface DashboardData {
  profile: Profile;
  family: Family | null;
  wallets: Wallet[];
  recent_transactions: Transaction[];
  request_summary: RequestSummary;
  pending_requests_count: number;
  unread_notification_count: number;
}

export interface AuditLog {
  id: number;
  actor_device: string | null;
  family_id: number | null;
  action: string;
  resource_type: string;
  resource_id: string | null;
  metadata: string | null;
  created_at: string;
}

export const REQUEST_CATEGORIES: { value: RequestCategory; label: string }[] = [
  { value: "food", label: "食物" },
  { value: "transport", label: "交通" },
  { value: "education", label: "教育" },
  { value: "entertainment", label: "娛樂" },
  { value: "clothing", label: "服飾" },
  { value: "health", label: "醫療" },
  { value: "other", label: "其他" },
];

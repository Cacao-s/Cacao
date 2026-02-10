-- Cacao database schema v1
-- Spec: docs/spec.claude.improve.md Section 5

-- 5.2 profiles
CREATE TABLE IF NOT EXISTS profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    display_name TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('giver','baby')),
    locale TEXT NOT NULL DEFAULT 'zh-TW',
    theme TEXT NOT NULL DEFAULT 'default',
    pin_hash TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 5.3 families
CREATE TABLE IF NOT EXISTS families (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    name TEXT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'TWD',
    timezone TEXT NOT NULL DEFAULT 'Asia/Taipei',
    created_by_device TEXT NOT NULL,
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 5.4 family_members
CREATE TABLE IF NOT EXISTS family_members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    family_id INTEGER NOT NULL,
    profile_uuid TEXT NOT NULL,
    family_role TEXT NOT NULL CHECK (family_role IN ('giver','baby','viewer')),
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','removed')),
    joined_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (family_id, profile_uuid),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE
);

-- 5.5 paired_devices
CREATE TABLE IF NOT EXISTS paired_devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_uuid TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('giver','baby')),
    family_id INTEGER NOT NULL,
    last_seen_at TEXT,
    last_sync_at TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE
);

-- 5.6 wallets
CREATE TABLE IF NOT EXISTS wallets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    family_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('cash','bank','card','virtual')),
    balance_cents INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'TWD',
    warning_threshold_cents INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','archived')),
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    UNIQUE (family_id, name)
);

-- 5.7 allowances
CREATE TABLE IF NOT EXISTS allowances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    family_id INTEGER NOT NULL,
    giver_member_id INTEGER NOT NULL,
    receiver_member_id INTEGER NOT NULL,
    wallet_id INTEGER NOT NULL,
    amount_cents INTEGER NOT NULL,
    frequency TEXT NOT NULL CHECK (frequency IN ('daily','weekly','biweekly','monthly','custom')),
    interval_count INTEGER NOT NULL DEFAULT 1,
    next_run_at TEXT,
    last_run_at TEXT,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','paused','archived')),
    notes TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE
);

-- 5.8 requests
CREATE TABLE IF NOT EXISTS requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    family_id INTEGER NOT NULL,
    requester_member_id INTEGER NOT NULL,
    wallet_id INTEGER NOT NULL,
    amount_cents INTEGER NOT NULL,
    category TEXT CHECK (category IN ('food','transport','education','entertainment','clothing','health','other')),
    notes TEXT,
    attachment_url TEXT,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft','pending','approved','rejected','cancelled')),
    decision_by_member_id INTEGER,
    decision_at TEXT,
    rejection_reason TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_requests_family_status ON requests(family_id, status);

-- 5.9 transactions
CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE DEFAULT (hex(randomblob(16))),
    family_id INTEGER NOT NULL,
    wallet_id INTEGER NOT NULL,
    type TEXT NOT NULL CHECK (type IN ('credit','debit')),
    amount_cents INTEGER NOT NULL,
    source_type TEXT NOT NULL CHECK (source_type IN ('allowance','request','manual','adjustment')),
    source_id INTEGER,
    category TEXT,
    occurred_at TEXT NOT NULL,
    notes TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (family_id) REFERENCES families(id) ON DELETE CASCADE,
    FOREIGN KEY (wallet_id) REFERENCES wallets(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_transactions_wallet_date ON transactions(wallet_id, occurred_at);
CREATE INDEX IF NOT EXISTS idx_transactions_family_date ON transactions(family_id, occurred_at);

-- 5.10 notifications
CREATE TABLE IF NOT EXISTS notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL CHECK (event_type IN (
        'request_submitted','request_approved','request_rejected',
        'allowance_disbursed','low_balance','member_joined'
    )),
    payload TEXT,
    is_read INTEGER NOT NULL DEFAULT 0,
    read_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 5.11 audit_logs
CREATE TABLE IF NOT EXISTS audit_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    actor_device TEXT,
    family_id INTEGER,
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    metadata TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 5.12 sync_state
CREATE TABLE IF NOT EXISTS sync_state (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

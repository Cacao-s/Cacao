-- Cacao schema version 1
-- 唯一來源：Android（SQLDelight）與 Phase 2 iOS（GRDB）共用。規則見 docs/data-model.md。
-- 版本號由各平台的 migration 執行器寫入 PRAGMA user_version，不寫在這個檔案裡。

CREATE TABLE member (
  id              TEXT NOT NULL PRIMARY KEY,
  role            TEXT NOT NULL CHECK (role IN ('parent', 'elder', 'child')),
  name            TEXT NOT NULL,
  birthday        TEXT,
  avatar_color    TEXT NOT NULL,
  pin_hash        TEXT,
  recovery_hash   TEXT,
  password_hash   TEXT,
  sort_order      INTEGER NOT NULL DEFAULT 0,
  created_at      TEXT NOT NULL,
  updated_at      TEXT NOT NULL,
  deleted_at      TEXT,
  CHECK (role <> 'child'  OR birthday IS NOT NULL),
  CHECK (role <> 'parent' OR pin_hash IS NOT NULL)
);

CREATE TABLE goal (
  id              TEXT NOT NULL PRIMARY KEY,
  child_id        TEXT NOT NULL REFERENCES member(id),
  name            TEXT NOT NULL,
  target_amount   INTEGER NOT NULL CHECK (target_amount > 0),
  priority        REAL NOT NULL,
  fill_enabled    INTEGER NOT NULL DEFAULT 1 CHECK (fill_enabled IN (0, 1)),
  created_by      TEXT NOT NULL REFERENCES member(id),
  created_at      TEXT NOT NULL,
  updated_at      TEXT NOT NULL,
  deleted_at      TEXT
);
CREATE INDEX goal_child ON goal(child_id, priority);

CREATE TABLE goal_event (
  id              TEXT NOT NULL PRIMARY KEY,
  goal_id         TEXT NOT NULL REFERENCES goal(id),
  kind            TEXT NOT NULL CHECK (kind IN ('confirmed', 'reverted', 'completed', 'target_changed')),
  amount          INTEGER,
  ledger_group_id TEXT,
  member_id       TEXT NOT NULL REFERENCES member(id),
  created_at      TEXT NOT NULL
);
CREATE INDEX goal_event_goal ON goal_event(goal_id, created_at);

CREATE TABLE allowance_schedule (
  id              TEXT NOT NULL PRIMARY KEY,
  child_id        TEXT NOT NULL REFERENCES member(id),
  source_id       TEXT NOT NULL REFERENCES member(id),
  amount          INTEGER NOT NULL CHECK (amount > 0),
  period          TEXT NOT NULL CHECK (period IN ('daily', 'weekly', 'biweekly', 'monthly')),
  weekday         INTEGER CHECK (weekday BETWEEN 1 AND 7),
  month_day       INTEGER CHECK (month_day BETWEEN 1 AND 31),
  active_from     TEXT NOT NULL,
  paused          INTEGER NOT NULL DEFAULT 0 CHECK (paused IN (0, 1)),
  created_at      TEXT NOT NULL,
  updated_at      TEXT NOT NULL,
  deleted_at      TEXT
);

CREATE TABLE task (
  id              TEXT NOT NULL PRIMARY KEY,
  batch_id        TEXT NOT NULL,
  child_id        TEXT NOT NULL REFERENCES member(id),
  title           TEXT NOT NULL,
  reward_amount   INTEGER NOT NULL CHECK (reward_amount > 0),
  issuer_id       TEXT NOT NULL REFERENCES member(id),
  created_by      TEXT NOT NULL REFERENCES member(id),
  created_at      TEXT NOT NULL,
  updated_at      TEXT NOT NULL,
  deleted_at      TEXT
);
CREATE INDEX task_child ON task(child_id);

CREATE TABLE ledger_entry (
  id              TEXT NOT NULL PRIMARY KEY,
  group_id        TEXT NOT NULL,
  child_id        TEXT NOT NULL REFERENCES member(id),
  account         TEXT NOT NULL CHECK (account IN ('savings', 'wallet')),
  kind            TEXT NOT NULL CHECK (kind IN (
                    'income_onetime',
                    'income_allowance',
                    'income_task',
                    'payout',
                    'wallet_expense',
                    'wallet_adjustment'
                  )),
  amount          INTEGER NOT NULL CHECK (amount <> 0),
  occurred_on     TEXT NOT NULL,
  source_id       TEXT REFERENCES member(id),
  goal_id         TEXT REFERENCES goal(id),
  note            TEXT,
  message         TEXT,
  allowance_id    TEXT REFERENCES allowance_schedule(id),
  allowance_due   TEXT,
  recorded_by     TEXT REFERENCES member(id),
  created_at      TEXT NOT NULL,
  UNIQUE (allowance_id, allowance_due)
);
CREATE INDEX ledger_child ON ledger_entry(child_id, account, occurred_on);
CREATE INDEX ledger_group ON ledger_entry(group_id);
CREATE INDEX ledger_goal ON ledger_entry(goal_id);

CREATE TABLE ledger_void (
  id              TEXT NOT NULL PRIMARY KEY,
  group_id        TEXT NOT NULL UNIQUE,
  voided_by       TEXT NOT NULL REFERENCES member(id),
  created_at      TEXT NOT NULL
);

CREATE TABLE task_completion (
  id              TEXT NOT NULL PRIMARY KEY,
  task_id         TEXT NOT NULL UNIQUE REFERENCES task(id),
  verified_by     TEXT NOT NULL REFERENCES member(id),
  ledger_group_id TEXT NOT NULL,
  completed_at    TEXT NOT NULL
);

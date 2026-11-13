-- Cacao schema version 2：孩子提議任務（Phase 2，discover WD-35）
-- 孩子新增的任務是 0 元，先放這裡；父母設定金額後轉成正式 task，原提議軟刪除。
-- Android：新安裝的建表寫在 sqldelight/Task.sq，這個檔案只負責 v1 → v2 升級，兩邊要一致。

CREATE TABLE task_proposal (
  id              TEXT NOT NULL PRIMARY KEY,
  child_id        TEXT NOT NULL REFERENCES member(id),
  title           TEXT NOT NULL,
  created_at      TEXT NOT NULL,
  updated_at      TEXT NOT NULL,
  deleted_at      TEXT
);
CREATE INDEX task_proposal_child ON task_proposal(child_id);

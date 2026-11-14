-- Cacao schema version 3：長輩是哪一位父母那邊的（Phase 2，discover WD-41；給之後的金錢貢獻報表用）
-- 誰（哪位父母）登入時新增的長輩，就算那位父母這邊的。「誰給的」不過濾，所有人都能選。
-- 沒有這張表的資料 = 還沒分邊（升級前就有的長輩），可在家庭成員頁按「算我這邊」。
-- Android：新安裝的建表寫在 sqldelight/Member.sq，這個檔案只負責 v2 → v3 升級，兩邊要一致。

CREATE TABLE elder_side (
  elder_id        TEXT NOT NULL PRIMARY KEY REFERENCES member(id),
  parent_id       TEXT NOT NULL REFERENCES member(id),
  created_at      TEXT NOT NULL,
  updated_at      TEXT NOT NULL
);

-- 只有一位父母的家庭，舊長輩一定是這位父母新增的，直接分給他
INSERT INTO elder_side(elder_id, parent_id, created_at, updated_at)
SELECT e.id, p.id, e.created_at, e.created_at
FROM member e, member p
WHERE e.role = 'elder' AND p.role = 'parent' AND p.deleted_at IS NULL
  AND (SELECT COUNT(*) FROM member WHERE role = 'parent' AND deleted_at IS NULL) = 1;

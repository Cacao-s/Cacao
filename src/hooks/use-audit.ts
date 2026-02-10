import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import type { AuditLog } from "@/lib/types";

export function useAuditLogs(familyId: number, limit = 50, offset = 0) {
  return useQuery({
    queryKey: ["audit-logs", familyId, limit, offset],
    queryFn: () =>
      invoke<AuditLog[]>("list_audit_logs", {
        familyId,
        limit,
        offset,
      }),
    enabled: familyId > 0,
  });
}

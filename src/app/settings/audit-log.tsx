import { useNavigate } from "react-router";
import { ArrowLeft, Shield } from "lucide-react";
import { Button } from "@/components/ui/button";
import { LoadingSkeleton } from "@/components/loading-skeleton";
import { EmptyState } from "@/components/empty-state";
import { useAuditLogs } from "@/hooks/use-audit";
import { useProfileStore } from "@/stores/profile-store";
import { formatDateTime } from "@/lib/format";

const actionLabels: Record<string, string> = {
  create_wallet: "建立錢包",
  archive_wallet: "封存錢包",
  create_request: "建立請款",
  submit_request: "提交請款",
  approve_request: "核准請款",
  reject_request: "駁回請款",
  cancel_request: "取消請款",
  create_allowance: "建立津貼",
  update_allowance: "更新津貼",
  pause_allowance: "暫停津貼",
  resume_allowance: "恢復津貼",
  execute_allowance: "發放津貼",
  create_transaction: "建立交易",
  setup_device: "設定裝置",
  create_family: "建立家庭",
  join_family: "加入家庭",
  remove_member: "移除成員",
  reset_device: "重設裝置",
};

const resourceLabels: Record<string, string> = {
  wallet: "錢包",
  request: "請款",
  allowance: "津貼",
  transaction: "交易",
  family: "家庭",
  profile: "裝置",
  family_member: "成員",
};

export function AuditLogPage() {
  const navigate = useNavigate();
  const family = useProfileStore((s) => s.family);
  const { data: logs, isLoading } = useAuditLogs(family?.id ?? 0);

  return (
    <div className="p-4">
      <div className="mb-4 flex items-center gap-2">
        <Button variant="ghost" size="icon" onClick={() => navigate(-1)}>
          <ArrowLeft className="size-5" />
        </Button>
        <h1 className="text-lg font-bold">審計日誌</h1>
      </div>

      {isLoading ? (
        <LoadingSkeleton />
      ) : !logs || logs.length === 0 ? (
        <EmptyState
          icon={<Shield className="size-8" />}
          title="尚無操作紀錄"
          description="所有操作都會記錄在這裡"
        />
      ) : (
        <div className="space-y-1">
          {logs.map((log) => (
            <div key={log.id} className="flex items-start gap-3 rounded-lg border p-3">
              <div className="flex-1 space-y-1">
                <p className="text-sm font-medium">{actionLabels[log.action] ?? log.action}</p>
                <div className="flex flex-wrap gap-2 text-xs text-muted-foreground">
                  <span>
                    {resourceLabels[log.resource_type] ?? log.resource_type}
                    {log.resource_id ? ` #${log.resource_id}` : ""}
                  </span>
                  {log.actor_device && <span>· {log.actor_device}</span>}
                </div>
                <p className="text-xs text-muted-foreground">{formatDateTime(log.created_at)}</p>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

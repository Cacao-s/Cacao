import { useEffect } from "react";
import { Wifi, WifiOff, Loader2, AlertCircle, Search } from "lucide-react";
import { cn } from "@/lib/utils";
import { useSyncStore, type SyncState } from "@/stores/sync-store";

const stateConfig: Record<SyncState, { icon: typeof Wifi; label: string; color: string }> = {
  connected: { icon: Wifi, label: "已連線", color: "text-green-500" },
  syncing: { icon: Loader2, label: "同步中", color: "text-blue-500" },
  discovering: { icon: Search, label: "搜尋中", color: "text-yellow-500" },
  disconnected: { icon: WifiOff, label: "未連線", color: "text-muted-foreground" },
  error: { icon: AlertCircle, label: "錯誤", color: "text-destructive" },
};

interface SyncIndicatorProps {
  className?: string;
  showLabel?: boolean;
}

export function SyncIndicator({ className, showLabel = false }: SyncIndicatorProps) {
  const { status, fetchStatus, triggerSync } = useSyncStore();

  useEffect(() => {
    fetchStatus();
    const interval = setInterval(fetchStatus, 10000);
    return () => clearInterval(interval);
  }, [fetchStatus]);

  const config = stateConfig[status.state] ?? stateConfig.disconnected;
  const Icon = config.icon;
  const isSyncing = status.state === "syncing";

  return (
    <button
      type="button"
      onClick={triggerSync}
      className={cn("flex items-center gap-1.5", className)}
      title={`${config.label}${status.peer_device_name ? ` - ${status.peer_device_name}` : ""}`}
    >
      <Icon className={cn("size-4", config.color, isSyncing && "animate-spin")} />
      {showLabel && <span className={cn("text-xs", config.color)}>{config.label}</span>}
    </button>
  );
}

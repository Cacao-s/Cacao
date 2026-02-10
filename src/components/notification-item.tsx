import { CheckCircle2, XCircle, Calendar, AlertTriangle, UserPlus, Receipt } from "lucide-react";
import { cn } from "@/lib/utils";
import { formatDateTime } from "@/lib/format";
import type { Notification } from "@/hooks/use-notifications";

const eventConfig: Record<string, { icon: typeof Receipt; label: string; color: string }> = {
  request_submitted: { icon: Receipt, label: "新請款", color: "text-blue-500" },
  request_approved: { icon: CheckCircle2, label: "請款已核准", color: "text-green-500" },
  request_rejected: { icon: XCircle, label: "請款已駁回", color: "text-destructive" },
  allowance_disbursed: { icon: Calendar, label: "津貼已發放", color: "text-green-500" },
  low_balance: { icon: AlertTriangle, label: "餘額偏低", color: "text-amber-500" },
  member_joined: { icon: UserPlus, label: "新成員加入", color: "text-blue-500" },
};

interface NotificationItemProps {
  notification: Notification;
  onClick?: () => void;
}

export function NotificationItem({ notification, onClick }: NotificationItemProps) {
  const config = eventConfig[notification.event_type] ?? {
    icon: Receipt,
    label: notification.event_type,
    color: "text-muted-foreground",
  };
  const Icon = config.icon;
  const isRead = notification.is_read === 1;

  let description = "";
  if (notification.payload) {
    try {
      const data = JSON.parse(notification.payload);
      if (data.amount_cents) {
        const amount = (data.amount_cents / 100).toLocaleString("zh-TW");
        description = `NT$${amount}`;
      }
      if (data.reason) {
        description += description ? ` — ${data.reason}` : data.reason;
      }
    } catch {
      // ignore parse errors
    }
  }

  return (
    <button
      type="button"
      className={cn(
        "flex w-full items-start gap-3 rounded-lg p-3 text-left transition-colors hover:bg-accent/50",
        !isRead && "bg-accent/20",
      )}
      onClick={onClick}
    >
      <div className={cn("mt-0.5", config.color)}>
        <Icon className="size-5" />
      </div>
      <div className="flex-1 space-y-0.5">
        <p className={cn("text-sm", !isRead && "font-semibold")}>{config.label}</p>
        {description && <p className="text-xs text-muted-foreground">{description}</p>}
        <p className="text-xs text-muted-foreground">{formatDateTime(notification.created_at)}</p>
      </div>
      {!isRead && <div className="mt-2 size-2 shrink-0 rounded-full bg-primary" />}
    </button>
  );
}

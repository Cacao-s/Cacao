import { useNavigate } from "react-router";
import { Bell, CheckCheck } from "lucide-react";
import { Button } from "@/components/ui/button";
import { NotificationItem } from "@/components/notification-item";
import { EmptyState } from "@/components/empty-state";
import { LoadingSkeleton } from "@/components/loading-skeleton";
import {
  useNotifications,
  useMarkRead,
  useMarkAllRead,
  useUnreadCount,
} from "@/hooks/use-notifications";

export function NotificationsPage() {
  const navigate = useNavigate();
  const { data: notifications, isLoading } = useNotifications();
  const { data: unreadCount } = useUnreadCount();
  const markRead = useMarkRead();
  const markAllRead = useMarkAllRead();

  function handleClick(notification: {
    id: number;
    event_type: string;
    payload: string | null;
    is_read: number;
  }) {
    if (notification.is_read === 0) {
      markRead.mutate(notification.id);
    }
    // Navigate to related page
    if (notification.payload) {
      try {
        const data = JSON.parse(notification.payload);
        if (data.request_id) {
          navigate(`/requests/${data.request_id}`);
          return;
        }
        if (data.wallet_id) {
          navigate(`/wallets/${data.wallet_id}`);
          return;
        }
      } catch {
        // ignore
      }
    }
  }

  if (isLoading) return <LoadingSkeleton count={4} />;

  return (
    <div className="flex flex-col gap-4 p-4">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold">通知</h1>
        {(unreadCount ?? 0) > 0 && (
          <Button variant="ghost" size="sm" onClick={() => markAllRead.mutate()}>
            <CheckCheck className="mr-1 size-4" />
            全部已讀
          </Button>
        )}
      </div>

      {!notifications || notifications.length === 0 ? (
        <EmptyState
          icon={<Bell className="size-12" />}
          title="沒有通知"
          description="通知會在請款、津貼、同步等事件發生時出現"
        />
      ) : (
        <div className="flex flex-col gap-1">
          {notifications.map((n) => (
            <NotificationItem key={n.id} notification={n} onClick={() => handleClick(n)} />
          ))}
        </div>
      )}
    </div>
  );
}

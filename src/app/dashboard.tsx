import { useNavigate } from "react-router";
import {
  Wallet,
  Receipt,
  Bell,
  Plus,
  AlertTriangle,
  ArrowRight,
  Clock,
  CheckCircle,
  XCircle,
} from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { AmountDisplay } from "@/components/amount-display";
import { TransactionItem } from "@/components/transaction-item";
import { LoadingSkeleton } from "@/components/loading-skeleton";
import { EmptyState } from "@/components/empty-state";
import { useDashboard } from "@/hooks/use-dashboard";

export function DashboardPage() {
  const { data, isLoading, error } = useDashboard();
  const navigate = useNavigate();

  if (isLoading) return <LoadingSkeleton />;
  if (error || !data) {
    return <div className="p-4 text-center text-destructive">載入失敗，請稍後再試</div>;
  }

  const isGiver = data.profile.role === "giver";

  return (
    <div className="space-y-4 p-4">
      {/* Greeting */}
      <div>
        <h1 className="text-xl font-bold">{data.profile.display_name}，你好</h1>
        <p className="text-sm text-muted-foreground">{data.family?.name ?? "尚未加入家庭"}</p>
      </div>

      {/* Pending requests alert (Giver) */}
      {isGiver && data.pending_requests_count > 0 && (
        <button
          className="flex w-full items-center gap-3 rounded-lg border border-amber-200 bg-amber-50 p-3 text-left dark:border-amber-800 dark:bg-amber-950"
          onClick={() => navigate("/requests")}
        >
          <AlertTriangle className="size-5 shrink-0 text-amber-500" />
          <div className="flex-1">
            <p className="text-sm font-medium">{data.pending_requests_count} 筆待審核請求</p>
            <p className="text-xs text-muted-foreground">點擊查看並審核</p>
          </div>
          <ArrowRight className="size-4 text-muted-foreground" />
        </button>
      )}

      {/* Unread notifications */}
      {data.unread_notification_count > 0 && (
        <button
          className="flex w-full items-center gap-3 rounded-lg border p-3 text-left"
          onClick={() => navigate("/notifications")}
        >
          <Bell className="size-5 shrink-0 text-primary" />
          <div className="flex-1">
            <p className="text-sm font-medium">{data.unread_notification_count} 則未讀通知</p>
          </div>
          <ArrowRight className="size-4 text-muted-foreground" />
        </button>
      )}

      {/* Wallets overview */}
      <section>
        <div className="mb-2 flex items-center justify-between">
          <h2 className="text-sm font-semibold text-muted-foreground">錢包</h2>
          <Button
            variant="ghost"
            size="sm"
            className="h-auto p-0 text-xs"
            onClick={() => navigate("/wallets")}
          >
            查看全部
          </Button>
        </div>
        {data.wallets.length === 0 ? (
          <Card>
            <CardContent>
              <EmptyState
                icon={<Wallet className="size-8" />}
                title="尚無錢包"
                description={isGiver ? "建立第一個錢包開始管理零用錢" : "等待家長建立錢包"}
                action={
                  isGiver ? (
                    <Button size="sm" onClick={() => navigate("/wallets")}>
                      <Plus className="mr-1 size-4" />
                      建立錢包
                    </Button>
                  ) : undefined
                }
              />
            </CardContent>
          </Card>
        ) : (
          <div className="grid gap-3">
            {data.wallets.map((wallet) => (
              <Card
                key={wallet.id}
                className="cursor-pointer transition-colors hover:bg-accent/50"
                onClick={() => navigate(`/wallets/${wallet.id}`)}
              >
                <CardContent className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className="flex size-10 items-center justify-center rounded-full bg-primary/10">
                      <Wallet className="size-5 text-primary" />
                    </div>
                    <div>
                      <p className="text-sm font-medium">{wallet.name}</p>
                      <p className="text-xs text-muted-foreground">
                        {wallet.type === "shared" ? "共用" : "個人"}
                      </p>
                    </div>
                  </div>
                  <AmountDisplay cents={wallet.balance_cents} className="text-base font-semibold" />
                </CardContent>
              </Card>
            ))}
          </div>
        )}
      </section>

      {/* Request summary */}
      <section>
        <div className="mb-2 flex items-center justify-between">
          <h2 className="text-sm font-semibold text-muted-foreground">請款概況</h2>
          <Button
            variant="ghost"
            size="sm"
            className="h-auto p-0 text-xs"
            onClick={() => navigate("/requests")}
          >
            查看全部
          </Button>
        </div>
        <div className="grid grid-cols-3 gap-2">
          <Card className="py-3">
            <CardContent className="flex flex-col items-center gap-1 p-0 px-2">
              <Clock className="size-4 text-amber-500" />
              <span className="text-lg font-bold">{data.request_summary.pending}</span>
              <span className="text-xs text-muted-foreground">待審核</span>
            </CardContent>
          </Card>
          <Card className="py-3">
            <CardContent className="flex flex-col items-center gap-1 p-0 px-2">
              <CheckCircle className="size-4 text-green-500" />
              <span className="text-lg font-bold">{data.request_summary.approved}</span>
              <span className="text-xs text-muted-foreground">已核准</span>
            </CardContent>
          </Card>
          <Card className="py-3">
            <CardContent className="flex flex-col items-center gap-1 p-0 px-2">
              <XCircle className="size-4 text-destructive" />
              <span className="text-lg font-bold">{data.request_summary.rejected}</span>
              <span className="text-xs text-muted-foreground">已駁回</span>
            </CardContent>
          </Card>
        </div>
      </section>

      {/* Recent transactions */}
      <section>
        <div className="mb-2 flex items-center justify-between">
          <h2 className="text-sm font-semibold text-muted-foreground">最近交易</h2>
          <Button
            variant="ghost"
            size="sm"
            className="h-auto p-0 text-xs"
            onClick={() => navigate("/transactions")}
          >
            查看全部
          </Button>
        </div>
        <Card>
          <CardContent>
            {data.recent_transactions.length === 0 ? (
              <p className="py-4 text-center text-sm text-muted-foreground">尚無交易紀錄</p>
            ) : (
              <div className="divide-y">
                {data.recent_transactions.map((tx) => (
                  <TransactionItem key={tx.id} transaction={tx} />
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      </section>

      {/* Quick actions */}
      <section>
        <h2 className="mb-2 text-sm font-semibold text-muted-foreground">快速操作</h2>
        <div className="grid grid-cols-2 gap-2">
          {!isGiver && (
            <Button
              variant="outline"
              className="h-auto flex-col gap-1 py-3"
              onClick={() => navigate("/requests/new")}
            >
              <Receipt className="size-5" />
              <span className="text-xs">新增請款</span>
            </Button>
          )}
          <Button
            variant="outline"
            className="h-auto flex-col gap-1 py-3"
            onClick={() => navigate("/wallets")}
          >
            <Wallet className="size-5" />
            <span className="text-xs">管理錢包</span>
          </Button>
          <Button
            variant="outline"
            className="h-auto flex-col gap-1 py-3"
            onClick={() => navigate("/requests")}
          >
            <Receipt className="size-5" />
            <span className="text-xs">{isGiver ? "審核請款" : "我的請款"}</span>
          </Button>
          <Button
            variant="outline"
            className="h-auto flex-col gap-1 py-3"
            onClick={() => navigate("/notifications")}
          >
            <Bell className="size-5" />
            <span className="text-xs">通知中心</span>
          </Button>
        </div>
      </section>
    </div>
  );
}

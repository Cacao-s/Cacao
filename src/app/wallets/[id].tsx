import { useParams, useNavigate } from "react-router";
import {
  ArrowLeft,
  Banknote,
  Building2,
  CreditCard,
  Smartphone,
  AlertTriangle,
  Archive,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { AmountDisplay } from "@/components/amount-display";
import { LoadingSkeleton } from "@/components/loading-skeleton";
import { useWallet, useArchiveWallet } from "@/hooks/use-wallets";
import { useWalletTransactions } from "@/hooks/use-requests";
import { useProfileStore } from "@/stores/profile-store";
import { formatAmount, formatDateTime } from "@/lib/format";

const walletTypeConfig: Record<string, { icon: typeof Banknote; label: string }> = {
  cash: { icon: Banknote, label: "現金" },
  bank: { icon: Building2, label: "銀行" },
  card: { icon: CreditCard, label: "信用卡" },
  virtual: { icon: Smartphone, label: "電子錢包" },
};

export function WalletDetailPage() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const profile = useProfileStore((s) => s.profile);
  const isGiver = profile?.role === "giver";

  const walletId = Number(id);
  const { data: wallet, isLoading, isError } = useWallet(walletId);
  const { data: transactions } = useWalletTransactions(walletId, 20);
  const archiveWallet = useArchiveWallet();

  function handleArchive() {
    archiveWallet.mutate(walletId, {
      onSuccess: () => navigate("/wallets"),
    });
  }

  if (isLoading) {
    return (
      <div className="p-4">
        <LoadingSkeleton />
      </div>
    );
  }

  if (isError || !wallet) {
    return (
      <div className="flex flex-col items-center justify-center gap-4 p-8">
        <p className="text-sm text-muted-foreground">找不到此錢包</p>
        <Button variant="outline" onClick={() => navigate("/wallets")}>
          返回錢包列表
        </Button>
      </div>
    );
  }

  const config = walletTypeConfig[wallet.type] ?? walletTypeConfig.cash;
  const Icon = config.icon;
  const isLowBalance =
    wallet.warning_threshold_cents > 0 && wallet.balance_cents < wallet.warning_threshold_cents;
  const isArchived = wallet.status === "archived";

  return (
    <div className="space-y-4 p-4">
      {/* Header */}
      <div className="flex items-center gap-2">
        <Button variant="ghost" size="icon" onClick={() => navigate("/wallets")}>
          <ArrowLeft className="size-5" />
        </Button>
        <h1 className="flex-1 text-xl font-bold">{wallet.name}</h1>
        {isArchived && <Badge variant="secondary">已封存</Badge>}
      </div>

      {/* Balance card */}
      <Card>
        <CardContent className="flex flex-col items-center gap-3 py-8">
          <div className="flex size-14 items-center justify-center rounded-full bg-muted">
            <Icon className="size-7 text-muted-foreground" />
          </div>
          <span className="text-sm text-muted-foreground">{config.label}</span>
          <AmountDisplay
            cents={wallet.balance_cents}
            currency={wallet.currency}
            className="text-3xl font-bold"
          />
          {isLowBalance && !isArchived && (
            <div className="flex items-center gap-1 text-sm text-amber-600 dark:text-amber-400">
              <AlertTriangle className="size-4" />
              <span>餘額偏低</span>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Details */}
      <Card>
        <CardHeader>
          <CardTitle>詳細資訊</CardTitle>
        </CardHeader>
        <CardContent className="space-y-3">
          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">幣別</span>
            <span>{wallet.currency}</span>
          </div>
          <Separator />
          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">低餘額警告</span>
            <span>
              {wallet.warning_threshold_cents > 0
                ? formatAmount(wallet.warning_threshold_cents, wallet.currency)
                : "未設定"}
            </span>
          </div>
          <Separator />
          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">狀態</span>
            <span>{isArchived ? "已封存" : "啟用中"}</span>
          </div>
        </CardContent>
      </Card>

      {/* Transactions */}
      <Card>
        <CardHeader>
          <CardTitle>交易紀錄</CardTitle>
        </CardHeader>
        <CardContent>
          {!transactions || transactions.length === 0 ? (
            <p className="text-sm text-muted-foreground">尚無交易紀錄</p>
          ) : (
            <div className="flex flex-col gap-3">
              {transactions.map((tx) => (
                <div key={tx.id} className="flex items-center justify-between text-sm">
                  <div className="space-y-0.5">
                    <p className="font-medium">
                      {tx.source_type === "request"
                        ? "請款支出"
                        : tx.source_type === "allowance"
                          ? "津貼發放"
                          : tx.source_type === "manual"
                            ? "手動記帳"
                            : "調整"}
                    </p>
                    <p className="text-xs text-muted-foreground">
                      {formatDateTime(tx.occurred_at)}
                    </p>
                  </div>
                  <AmountDisplay
                    cents={tx.type === "debit" ? -tx.amount_cents : tx.amount_cents}
                    showSign
                    className="font-medium"
                  />
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Archive action for giver */}
      {isGiver && !isArchived && (
        <AlertDialog>
          <AlertDialogTrigger asChild>
            <Button variant="destructive" className="w-full" disabled={archiveWallet.isPending}>
              <Archive className="size-4" />
              {archiveWallet.isPending ? "封存中..." : "封存錢包"}
            </Button>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>確認封存</AlertDialogTitle>
              <AlertDialogDescription>
                封存後將無法再對此錢包進行交易。此操作無法復原。
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>取消</AlertDialogCancel>
              <AlertDialogAction variant="destructive" onClick={handleArchive}>
                確認封存
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      )}
    </div>
  );
}

import { useNavigate } from "react-router";
import { Banknote, Building2, CreditCard, Smartphone, AlertTriangle } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { AmountDisplay } from "@/components/amount-display";
import type { Wallet } from "@/lib/types";

const walletTypeConfig: Record<string, { icon: typeof Banknote; label: string }> = {
  cash: { icon: Banknote, label: "現金" },
  bank: { icon: Building2, label: "銀行" },
  card: { icon: CreditCard, label: "信用卡" },
  virtual: { icon: Smartphone, label: "電子錢包" },
};

interface WalletCardProps {
  wallet: Wallet;
}

export function WalletCard({ wallet }: WalletCardProps) {
  const navigate = useNavigate();
  const config = walletTypeConfig[wallet.type] ?? walletTypeConfig.cash;
  const Icon = config.icon;
  const isLowBalance =
    wallet.warning_threshold_cents > 0 && wallet.balance_cents < wallet.warning_threshold_cents;
  const isArchived = wallet.status === "archived";

  return (
    <Card
      className="cursor-pointer transition-colors hover:bg-accent/50"
      onClick={() => navigate(`/wallets/${wallet.id}`)}
    >
      <CardContent className="flex items-center gap-4">
        <div className="flex size-10 shrink-0 items-center justify-center rounded-full bg-muted">
          <Icon className="size-5 text-muted-foreground" />
        </div>
        <div className="flex-1 space-y-1">
          <div className="flex items-center gap-2">
            <span className="font-medium">{wallet.name}</span>
            {isArchived && <Badge variant="secondary">已封存</Badge>}
          </div>
          <AmountDisplay
            cents={wallet.balance_cents}
            currency={wallet.currency}
            className="text-xl font-bold"
          />
          {isLowBalance && !isArchived && (
            <div className="flex items-center gap-1 text-xs text-amber-600 dark:text-amber-400">
              <AlertTriangle className="size-3" />
              <span>餘額偏低</span>
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  );
}

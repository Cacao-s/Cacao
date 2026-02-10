import { ArrowDownLeft, ArrowUpRight, Calendar, Receipt, Pencil, Settings } from "lucide-react";
import { AmountDisplay } from "@/components/amount-display";
import { formatDateTime } from "@/lib/format";
import type { Transaction } from "@/lib/types";

const sourceConfig: Record<string, { icon: typeof Receipt; label: string }> = {
  allowance: { icon: Calendar, label: "津貼發放" },
  request: { icon: Receipt, label: "請款支出" },
  manual: { icon: Pencil, label: "手動記帳" },
  adjustment: { icon: Settings, label: "調整" },
};

interface TransactionItemProps {
  transaction: Transaction;
}

export function TransactionItem({ transaction }: TransactionItemProps) {
  const isCredit = transaction.type === "credit";
  const config = sourceConfig[transaction.source_type] ?? sourceConfig.manual;
  const Icon = config.icon;
  const DirectionIcon = isCredit ? ArrowDownLeft : ArrowUpRight;

  return (
    <div className="flex items-center gap-3 py-2">
      <div className="flex size-10 shrink-0 items-center justify-center rounded-full bg-muted">
        <Icon className="size-5 text-muted-foreground" />
      </div>
      <div className="flex-1 space-y-0.5">
        <div className="flex items-center gap-1.5">
          <DirectionIcon className={`size-3 ${isCredit ? "text-green-500" : "text-destructive"}`} />
          <span className="text-sm font-medium">{config.label}</span>
        </div>
        <p className="text-xs text-muted-foreground">{formatDateTime(transaction.occurred_at)}</p>
      </div>
      <AmountDisplay
        cents={isCredit ? transaction.amount_cents : -transaction.amount_cents}
        showSign
        className="text-sm font-medium"
      />
    </div>
  );
}

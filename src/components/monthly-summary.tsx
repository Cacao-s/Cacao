import { TrendingUp, TrendingDown, Minus } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { AmountDisplay } from "@/components/amount-display";
import type { MonthlySummary as MonthlySummaryData } from "@/hooks/use-transactions";

interface MonthlySummaryProps {
  summary: MonthlySummaryData;
  year: number;
  month: number;
}

export function MonthlySummary({ summary, year, month }: MonthlySummaryProps) {
  const monthLabel = `${year} 年 ${month} 月`;

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-sm text-muted-foreground">{monthLabel} 統計</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="grid grid-cols-3 gap-4 text-center">
          <div className="space-y-1">
            <TrendingUp className="mx-auto size-4 text-green-500" />
            <p className="text-xs text-muted-foreground">入帳</p>
            <AmountDisplay cents={summary.total_credit_cents} className="text-sm font-semibold" />
          </div>
          <div className="space-y-1">
            <TrendingDown className="mx-auto size-4 text-destructive" />
            <p className="text-xs text-muted-foreground">出帳</p>
            <AmountDisplay cents={summary.total_debit_cents} className="text-sm font-semibold" />
          </div>
          <div className="space-y-1">
            <Minus className="mx-auto size-4 text-muted-foreground" />
            <p className="text-xs text-muted-foreground">淨變動</p>
            <AmountDisplay
              cents={summary.net_change_cents}
              showSign
              className="text-sm font-semibold"
            />
          </div>
        </div>
        <p className="mt-2 text-center text-xs text-muted-foreground">
          共 {summary.transaction_count} 筆交易
        </p>
      </CardContent>
    </Card>
  );
}

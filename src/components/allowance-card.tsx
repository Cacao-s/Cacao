import { Pause, Play, Calendar } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { AmountDisplay } from "@/components/amount-display";
import { formatDate } from "@/lib/format";
import type { Allowance } from "@/hooks/use-allowances";

const frequencyLabels: Record<string, string> = {
  daily: "每日",
  weekly: "每週",
  biweekly: "每兩週",
  monthly: "每月",
  custom: "自訂",
};

const statusConfig: Record<
  string,
  { label: string; variant: "default" | "secondary" | "outline" }
> = {
  active: { label: "啟用中", variant: "default" },
  paused: { label: "已暫停", variant: "secondary" },
  archived: { label: "已封存", variant: "outline" },
};

interface AllowanceCardProps {
  allowance: Allowance;
  isGiver: boolean;
  onPause?: (id: number) => void;
  onResume?: (id: number) => void;
}

export function AllowanceCard({ allowance, isGiver, onPause, onResume }: AllowanceCardProps) {
  const freq = frequencyLabels[allowance.frequency] ?? allowance.frequency;
  const config = statusConfig[allowance.status] ?? statusConfig.active;
  const isPaused = allowance.status === "paused";
  const isActive = allowance.status === "active";

  return (
    <Card>
      <CardContent className="flex items-center gap-4">
        <div className="flex size-10 shrink-0 items-center justify-center rounded-full bg-muted">
          <Calendar className="size-5 text-muted-foreground" />
        </div>
        <div className="flex-1 space-y-1">
          <div className="flex items-center justify-between">
            <AmountDisplay cents={allowance.amount_cents} className="text-lg font-bold" />
            <Badge variant={config.variant}>{config.label}</Badge>
          </div>
          <p className="text-sm text-muted-foreground">{freq}</p>
          {allowance.next_run_at && isActive && (
            <p className="text-xs text-muted-foreground">
              下次發放：{formatDate(allowance.next_run_at)}
            </p>
          )}
        </div>
        {isGiver && (isActive || isPaused) && (
          <Button
            variant="ghost"
            size="icon"
            onClick={(e) => {
              e.stopPropagation();
              if (isPaused) onResume?.(allowance.id);
              else onPause?.(allowance.id);
            }}
          >
            {isPaused ? <Play className="size-4" /> : <Pause className="size-4" />}
          </Button>
        )}
      </CardContent>
    </Card>
  );
}

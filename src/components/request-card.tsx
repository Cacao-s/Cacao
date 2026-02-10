import { useNavigate } from "react-router";
import {
  UtensilsCrossed,
  Bus,
  GraduationCap,
  Gamepad2,
  Shirt,
  HeartPulse,
  MoreHorizontal,
} from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";
import { AmountDisplay } from "@/components/amount-display";
import { RequestStatusBadge } from "@/components/request-status-badge";
import { formatDate } from "@/lib/format";
import type { Request as AppRequest } from "@/lib/types";

const categoryConfig: Record<string, { icon: typeof UtensilsCrossed; label: string }> = {
  food: { icon: UtensilsCrossed, label: "食物" },
  transport: { icon: Bus, label: "交通" },
  education: { icon: GraduationCap, label: "教育" },
  entertainment: { icon: Gamepad2, label: "娛樂" },
  clothing: { icon: Shirt, label: "服飾" },
  health: { icon: HeartPulse, label: "醫療" },
  other: { icon: MoreHorizontal, label: "其他" },
};

interface RequestCardProps {
  request: AppRequest;
}

export function RequestCard({ request }: RequestCardProps) {
  const navigate = useNavigate();
  const config = request.category
    ? (categoryConfig[request.category] ?? categoryConfig.other)
    : categoryConfig.other;
  const Icon = config.icon;

  return (
    <Card
      className="cursor-pointer transition-colors hover:bg-accent/50"
      onClick={() => navigate(`/requests/${request.id}`)}
    >
      <CardContent className="flex items-center gap-4">
        <div className="flex size-10 shrink-0 items-center justify-center rounded-full bg-muted">
          <Icon className="size-5 text-muted-foreground" />
        </div>
        <div className="flex-1 space-y-1">
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">{config.label}</span>
            <RequestStatusBadge status={request.status} />
          </div>
          <AmountDisplay cents={request.amount_cents} className="text-lg font-bold" />
          {request.notes && (
            <p className="truncate text-sm text-muted-foreground">{request.notes}</p>
          )}
          <p className="text-xs text-muted-foreground">{formatDate(request.created_at)}</p>
        </div>
      </CardContent>
    </Card>
  );
}

import { Badge } from "@/components/ui/badge";
import type { RequestStatus } from "@/lib/types";

const statusConfig: Record<
  RequestStatus,
  { label: string; variant: "default" | "secondary" | "destructive" | "outline" }
> = {
  draft: { label: "草稿", variant: "outline" },
  pending: { label: "待審核", variant: "default" },
  approved: { label: "已核准", variant: "secondary" },
  rejected: { label: "已駁回", variant: "destructive" },
  cancelled: { label: "已取消", variant: "outline" },
};

interface RequestStatusBadgeProps {
  status: string;
}

export function RequestStatusBadge({ status }: RequestStatusBadgeProps) {
  const config = statusConfig[status as RequestStatus] ?? {
    label: status,
    variant: "outline" as const,
  };

  return <Badge variant={config.variant}>{config.label}</Badge>;
}

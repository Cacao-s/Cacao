import { formatAmount } from "@/lib/format";
import { cn } from "@/lib/utils";

interface AmountDisplayProps {
  cents: number;
  currency?: string;
  className?: string;
  showSign?: boolean;
}

export function AmountDisplay({
  cents,
  currency = "TWD",
  className,
  showSign = false,
}: AmountDisplayProps) {
  const formatted = formatAmount(Math.abs(cents), currency);
  const sign = showSign && cents !== 0 ? (cents > 0 ? "+" : "-") : "";

  return (
    <span className={cn(cents < 0 && "text-destructive", className)}>
      {sign}
      {formatted}
    </span>
  );
}

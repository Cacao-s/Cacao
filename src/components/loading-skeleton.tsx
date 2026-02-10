export function LoadingSkeleton({ count = 3 }: { count?: number } = {}) {
  return (
    <div className="grid gap-4 p-4">
      {Array.from({ length: count }, (_, i) => i + 1).map((i) => (
        <div key={i} className="rounded-xl border bg-card p-6 shadow-sm">
          <div className="space-y-3">
            <div className="h-4 w-1/3 animate-pulse rounded bg-muted" />
            <div className="h-8 w-1/2 animate-pulse rounded bg-muted" />
            <div className="h-3 w-1/4 animate-pulse rounded bg-muted" />
          </div>
        </div>
      ))}
    </div>
  );
}

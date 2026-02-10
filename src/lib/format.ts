export function formatAmount(cents: number, currency = "TWD"): string {
  return new Intl.NumberFormat("zh-TW", {
    style: "currency",
    currency,
    minimumFractionDigits: 0,
  }).format(cents / 100);
}

export function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString("zh-TW");
}

export function formatDateTime(iso: string): string {
  return new Date(iso).toLocaleString("zh-TW");
}

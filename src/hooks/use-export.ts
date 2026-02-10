import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

interface ExportParams {
  familyId: number;
  dateFrom?: string;
  dateTo?: string;
  walletId?: number;
}

export function useExportCsv() {
  return useMutation({
    mutationFn: ({ familyId, dateFrom, dateTo, walletId }: ExportParams) =>
      invoke<string>("export_csv", {
        familyId,
        dateFrom: dateFrom ?? null,
        dateTo: dateTo ?? null,
        walletId: walletId ?? null,
      }),
  });
}

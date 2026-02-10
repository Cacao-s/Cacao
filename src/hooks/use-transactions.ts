import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import type { Transaction } from "@/lib/types";

interface TransactionFilters {
  date_from?: string | null;
  date_to?: string | null;
  wallet_id?: number | null;
  transaction_type?: string | null;
  source_type?: string | null;
  limit?: number | null;
  offset?: number | null;
}

interface MonthlySummary {
  total_credit_cents: number;
  total_debit_cents: number;
  net_change_cents: number;
  transaction_count: number;
}

export type { TransactionFilters, MonthlySummary };

export function useTransactions(familyId: number | undefined, filters?: TransactionFilters) {
  return useQuery({
    queryKey: ["transactions", familyId, filters],
    queryFn: () =>
      invoke<Transaction[]>("list_transactions", {
        familyId,
        filters: filters ?? {},
      }),
    enabled: !!familyId,
  });
}

export function useMonthlySummary(familyId: number | undefined, year: number, month: number) {
  return useQuery({
    queryKey: ["monthly-summary", familyId, year, month],
    queryFn: () =>
      invoke<MonthlySummary>("get_monthly_summary", {
        familyId,
        year,
        month,
      }),
    enabled: !!familyId,
  });
}

export function useCreateManualTransaction() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: {
      family_id: number;
      wallet_id: number;
      transaction_type: string;
      amount_cents: number;
      category?: string;
      notes?: string;
    }) => invoke<Transaction>("create_manual_transaction", { params }),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["transactions"] });
      qc.invalidateQueries({ queryKey: ["wallets"] });
      qc.invalidateQueries({ queryKey: ["wallet"] });
      qc.invalidateQueries({ queryKey: ["monthly-summary"] });
    },
  });
}

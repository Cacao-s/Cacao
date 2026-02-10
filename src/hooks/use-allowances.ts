import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

interface Allowance {
  id: number;
  uuid: string;
  family_id: number;
  giver_member_id: number;
  receiver_member_id: number;
  wallet_id: number;
  amount_cents: number;
  frequency: string;
  interval_count: number;
  next_run_at: string | null;
  last_run_at: string | null;
  status: string;
  notes: string | null;
  sync_version: number;
  last_synced_at: string | null;
  is_deleted: number;
  created_at: string;
  updated_at: string;
}

export type { Allowance };

export function useAllowances(familyId: number | undefined) {
  return useQuery({
    queryKey: ["allowances", familyId],
    queryFn: () => invoke<Allowance[]>("list_allowances", { familyId }),
    enabled: !!familyId,
  });
}

export function useCreateAllowance() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: {
      family_id: number;
      giver_member_id: number;
      receiver_member_id: number;
      wallet_id: number;
      amount_cents: number;
      frequency: string;
      interval_count?: number;
      notes?: string;
    }) => invoke<Allowance>("create_allowance", { params }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["allowances"] }),
  });
}

export function usePauseAllowance() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => invoke<void>("pause_allowance", { id }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["allowances"] }),
  });
}

export function useResumeAllowance() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => invoke<void>("resume_allowance", { id }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["allowances"] }),
  });
}

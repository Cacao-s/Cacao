import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import type { Wallet } from "@/lib/types";

export function useWallets(familyId: number | undefined) {
  return useQuery({
    queryKey: ["wallets", familyId],
    queryFn: () => invoke<Wallet[]>("list_wallets", { familyId }),
    enabled: !!familyId,
  });
}

export function useWallet(id: number) {
  return useQuery({
    queryKey: ["wallet", id],
    queryFn: () => invoke<Wallet>("get_wallet", { id }),
  });
}

export function useCreateWallet() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: {
      family_id: number;
      name: string;
      wallet_type: string;
      initial_balance_cents?: number;
    }) => invoke<Wallet>("create_wallet", { params }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["wallets"] }),
  });
}

export function useArchiveWallet() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => invoke<void>("archive_wallet", { id }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["wallets"] }),
  });
}

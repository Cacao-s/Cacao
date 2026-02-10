import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import type { Request as AppRequest, Transaction } from "@/lib/types";

export function useRequests(familyId: number | undefined, status?: string) {
  return useQuery({
    queryKey: ["requests", familyId, status],
    queryFn: () =>
      invoke<AppRequest[]>("list_requests", {
        familyId,
        status: status || null,
      }),
    enabled: !!familyId,
  });
}

export function useRequest(id: number) {
  return useQuery({
    queryKey: ["request", id],
    queryFn: () => invoke<AppRequest>("get_request", { id }),
  });
}

export function useCreateRequest() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: {
      family_id: number;
      requester_member_id: number;
      wallet_id: number;
      amount_cents: number;
      category?: string;
      notes?: string;
    }) => invoke<AppRequest>("create_request", { params }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["requests"] }),
  });
}

export function useSubmitRequest() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => invoke<AppRequest>("submit_request", { id }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["requests"] }),
  });
}

export function useApproveRequest() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: { id: number; approverMemberId: number }) =>
      invoke<AppRequest>("approve_request", {
        id: params.id,
        approverMemberId: params.approverMemberId,
      }),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["requests"] });
      qc.invalidateQueries({ queryKey: ["wallets"] });
      qc.invalidateQueries({ queryKey: ["wallet"] });
    },
  });
}

export function useRejectRequest() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: { id: number; reason: string; rejectorMemberId: number }) =>
      invoke<AppRequest>("reject_request", {
        id: params.id,
        reason: params.reason,
        rejectorMemberId: params.rejectorMemberId,
      }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["requests"] }),
  });
}

export function useCancelRequest() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => invoke<AppRequest>("cancel_request", { id }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["requests"] }),
  });
}

export function useWalletTransactions(walletId: number, limit?: number) {
  return useQuery({
    queryKey: ["transactions", walletId, limit],
    queryFn: () =>
      invoke<Transaction[]>("list_wallet_transactions", {
        walletId,
        limit: limit ?? null,
      }),
  });
}

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export function useHasPin() {
  return useQuery({
    queryKey: ["has-pin"],
    queryFn: () => invoke<boolean>("has_pin"),
  });
}

export function useSetPin() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (pin: string) => invoke("set_pin", { pin }),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["has-pin"] }),
  });
}

export function useVerifyPin() {
  return useMutation({
    mutationFn: (pin: string) => invoke<boolean>("verify_pin", { pin }),
  });
}

export function useRemovePin() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: () => invoke("remove_pin"),
    onSuccess: () => qc.invalidateQueries({ queryKey: ["has-pin"] }),
  });
}

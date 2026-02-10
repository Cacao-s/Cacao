import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import type { DashboardData } from "@/lib/types";

export function useDashboard() {
  return useQuery({
    queryKey: ["dashboard"],
    queryFn: () => invoke<DashboardData>("get_dashboard_data"),
    refetchInterval: 30_000,
  });
}

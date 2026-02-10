import { Navigate, Outlet } from "react-router";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

export function SetupGuard() {
  const { data: isSetup, isLoading } = useQuery({
    queryKey: ["is-setup"],
    queryFn: () => invoke<boolean>("is_device_setup"),
  });

  if (isLoading) {
    return (
      <div className="flex h-screen items-center justify-center">
        <div className="animate-spin h-8 w-8 border-4 border-primary border-t-transparent rounded-full" />
      </div>
    );
  }

  if (!isSetup) return <Navigate to="/setup" replace />;
  return <Outlet />;
}

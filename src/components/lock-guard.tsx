import { useState, useEffect, type ReactNode } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { LockScreen } from "@/app/lock-screen";

interface LockGuardProps {
  children: ReactNode;
}

export function LockGuard({ children }: LockGuardProps) {
  const [isLocked, setIsLocked] = useState(false);

  useEffect(() => {
    const unlisten = listen("tauri://resumed", async () => {
      try {
        const hasPin = await invoke<boolean>("has_pin");
        if (hasPin) setIsLocked(true);
      } catch {
        // If command fails, don't lock
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  if (isLocked) return <LockScreen onUnlock={() => setIsLocked(false)} />;
  return <>{children}</>;
}

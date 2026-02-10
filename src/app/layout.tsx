import { Outlet, useLocation, useNavigate } from "react-router";
import { Home, Wallet, Receipt, Bell, Settings, type LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";
import { SyncIndicator } from "@/components/sync-indicator";

interface TabItem {
  label: string;
  icon: LucideIcon;
  path: string;
}

const tabs: TabItem[] = [
  { label: "首頁", icon: Home, path: "/" },
  { label: "錢包", icon: Wallet, path: "/wallets" },
  { label: "請求", icon: Receipt, path: "/requests" },
  { label: "通知", icon: Bell, path: "/notifications" },
  { label: "設定", icon: Settings, path: "/settings" },
];

export function AppLayout() {
  const location = useLocation();
  const navigate = useNavigate();

  return (
    <div className="flex flex-col h-screen max-w-md mx-auto">
      {/* Header with sync indicator */}
      <header className="flex items-center justify-between border-b px-4 py-2 pt-[env(safe-area-inset-top)]">
        <span className="text-sm font-semibold">Cacao</span>
        <SyncIndicator showLabel />
      </header>

      {/* Content area */}
      <main className="flex-1 overflow-y-auto">
        <Outlet />
      </main>

      {/* Bottom tab bar */}
      <nav className="border-t bg-background pb-[env(safe-area-inset-bottom)]" role="tablist">
        <div className="flex items-center justify-around h-14">
          {tabs.map((tab) => {
            const isActive =
              tab.path === "/" ? location.pathname === "/" : location.pathname.startsWith(tab.path);

            return (
              <button
                key={tab.path}
                role="tab"
                aria-selected={isActive}
                className={cn(
                  "flex flex-col items-center justify-center gap-0.5 flex-1 h-full text-xs transition-colors",
                  isActive ? "text-primary" : "text-muted-foreground hover:text-foreground",
                )}
                onClick={() => navigate(tab.path)}
              >
                <tab.icon className="size-5" />
                <span>{tab.label}</span>
              </button>
            );
          })}
        </div>
      </nav>
    </div>
  );
}

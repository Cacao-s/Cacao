import { RouterProvider, createBrowserRouter } from "react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { SetupGuard } from "@/components/setup-guard";
import { LockGuard } from "@/components/lock-guard";
import { AppLayout } from "@/app/layout";
import { SetupPage } from "@/app/setup";
import { PairingPage } from "@/app/pairing";
import { DashboardPage } from "@/app/dashboard";
import { WalletsPage } from "@/app/wallets/index";
import { WalletDetailPage } from "@/app/wallets/[id]";
import { RequestsPage } from "@/app/requests/index";
import { NewRequestPage } from "@/app/requests/new";
import { RequestDetailPage } from "@/app/requests/[id]";
import { AllowancesPage } from "@/app/allowances/index";
import { TransactionsPage } from "@/app/transactions/index";
import { NotificationsPage } from "@/app/notifications/index";
import { SettingsPage } from "@/app/settings/index";
import { AuditLogPage } from "@/app/settings/audit-log";
import { FamilyPage } from "@/app/family";

const queryClient = new QueryClient();

const router = createBrowserRouter([
  { path: "/setup", element: <SetupPage /> },
  { path: "/pairing", element: <PairingPage /> },
  {
    element: <SetupGuard />,
    children: [
      {
        element: <AppLayout />,
        children: [
          {
            index: true,
            element: <DashboardPage />,
          },
          {
            path: "wallets",
            element: <WalletsPage />,
          },
          {
            path: "wallets/:id",
            element: <WalletDetailPage />,
          },
          {
            path: "allowances",
            element: <AllowancesPage />,
          },
          {
            path: "requests",
            element: <RequestsPage />,
          },
          {
            path: "requests/new",
            element: <NewRequestPage />,
          },
          {
            path: "requests/:id",
            element: <RequestDetailPage />,
          },
          {
            path: "transactions",
            element: <TransactionsPage />,
          },
          {
            path: "notifications",
            element: <NotificationsPage />,
          },
          {
            path: "settings",
            element: <SettingsPage />,
          },
          {
            path: "settings/audit-log",
            element: <AuditLogPage />,
          },
          {
            path: "family",
            element: <FamilyPage />,
          },
        ],
      },
    ],
  },
]);

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <LockGuard>
        <RouterProvider router={router} />
      </LockGuard>
    </QueryClientProvider>
  );
}

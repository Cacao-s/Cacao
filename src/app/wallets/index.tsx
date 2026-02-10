import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { Plus, Wallet, RefreshCw } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { WalletCard } from "@/components/wallet-card";
import { EmptyState } from "@/components/empty-state";
import { LoadingSkeleton } from "@/components/loading-skeleton";
import { useWallets, useCreateWallet } from "@/hooks/use-wallets";
import { useProfileStore } from "@/stores/profile-store";
import type { Family } from "@/lib/types";

export function WalletsPage() {
  const storeFamily = useProfileStore((s) => s.family);
  const profile = useProfileStore((s) => s.profile);
  const isGiver = profile?.role === "giver";

  const { data: fetchedFamily } = useQuery({
    queryKey: ["family"],
    queryFn: () => invoke<Family | null>("get_family"),
    enabled: !storeFamily,
  });

  const family = storeFamily ?? fetchedFamily;
  const { data: wallets, isLoading, isError, refetch } = useWallets(family?.id);

  const [dialogOpen, setDialogOpen] = useState(false);
  const [name, setName] = useState("");
  const [walletType, setWalletType] = useState("cash");
  const [initialBalance, setInitialBalance] = useState("");

  const createWallet = useCreateWallet();

  function handleCreate() {
    if (!family || !name.trim()) return;

    const balanceCents = initialBalance ? Math.round(parseFloat(initialBalance) * 100) : undefined;

    createWallet.mutate(
      {
        family_id: family.id,
        name: name.trim(),
        wallet_type: walletType,
        initial_balance_cents: balanceCents,
      },
      {
        onSuccess: () => {
          setDialogOpen(false);
          setName("");
          setWalletType("cash");
          setInitialBalance("");
        },
      },
    );
  }

  if (isError) {
    return (
      <div className="flex flex-col items-center justify-center gap-4 p-8">
        <p className="text-sm text-muted-foreground">載入錢包時發生錯誤</p>
        <Button variant="outline" onClick={() => refetch()}>
          <RefreshCw className="size-4" />
          重試
        </Button>
      </div>
    );
  }

  return (
    <div className="space-y-4 p-4">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold">錢包</h1>
      </div>

      {isLoading ? (
        <LoadingSkeleton />
      ) : !wallets || wallets.length === 0 ? (
        <EmptyState
          icon={<Wallet className="size-12" />}
          title="尚無錢包"
          description="建立第一個錢包來開始管理家庭財務"
          action={
            isGiver ? (
              <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
                <DialogTrigger asChild>
                  <Button>
                    <Plus className="size-4" />
                    建立錢包
                  </Button>
                </DialogTrigger>
                <CreateWalletDialogContent
                  name={name}
                  setName={setName}
                  walletType={walletType}
                  setWalletType={setWalletType}
                  initialBalance={initialBalance}
                  setInitialBalance={setInitialBalance}
                  isPending={createWallet.isPending}
                  onCreate={handleCreate}
                />
              </Dialog>
            ) : undefined
          }
        />
      ) : (
        <div className="grid gap-4">
          {wallets.map((wallet) => (
            <WalletCard key={wallet.id} wallet={wallet} />
          ))}
        </div>
      )}

      {/* Floating action button for giver when wallets exist */}
      {isGiver && wallets && wallets.length > 0 && (
        <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
          <DialogTrigger asChild>
            <Button size="icon" className="fixed bottom-20 right-4 size-14 rounded-full shadow-lg">
              <Plus className="size-6" />
            </Button>
          </DialogTrigger>
          <CreateWalletDialogContent
            name={name}
            setName={setName}
            walletType={walletType}
            setWalletType={setWalletType}
            initialBalance={initialBalance}
            setInitialBalance={setInitialBalance}
            isPending={createWallet.isPending}
            onCreate={handleCreate}
          />
        </Dialog>
      )}
    </div>
  );
}

function CreateWalletDialogContent({
  name,
  setName,
  walletType,
  setWalletType,
  initialBalance,
  setInitialBalance,
  isPending,
  onCreate,
}: {
  name: string;
  setName: (v: string) => void;
  walletType: string;
  setWalletType: (v: string) => void;
  initialBalance: string;
  setInitialBalance: (v: string) => void;
  isPending: boolean;
  onCreate: () => void;
}) {
  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>建立錢包</DialogTitle>
        <DialogDescription>為家庭建立一個新的錢包</DialogDescription>
      </DialogHeader>
      <div className="space-y-4 py-2">
        <div className="space-y-2">
          <Label htmlFor="wallet-name">名稱</Label>
          <Input
            id="wallet-name"
            placeholder="例如：零用錢"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
        <div className="space-y-2">
          <Label>類型</Label>
          <Select value={walletType} onValueChange={setWalletType}>
            <SelectTrigger className="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="cash">現金</SelectItem>
              <SelectItem value="bank">銀行</SelectItem>
              <SelectItem value="card">信用卡</SelectItem>
              <SelectItem value="virtual">電子錢包</SelectItem>
            </SelectContent>
          </Select>
        </div>
        <div className="space-y-2">
          <Label htmlFor="wallet-balance">初始餘額（選填）</Label>
          <Input
            id="wallet-balance"
            type="number"
            placeholder="0"
            min="0"
            step="1"
            value={initialBalance}
            onChange={(e) => setInitialBalance(e.target.value)}
          />
        </div>
      </div>
      <DialogFooter>
        <Button onClick={onCreate} disabled={!name.trim() || isPending}>
          {isPending ? "建立中..." : "建立"}
        </Button>
      </DialogFooter>
    </DialogContent>
  );
}

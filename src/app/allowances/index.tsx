import { useState } from "react";
import { Calendar, Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { AllowanceCard } from "@/components/allowance-card";
import { EmptyState } from "@/components/empty-state";
import { LoadingSkeleton } from "@/components/loading-skeleton";
import {
  useAllowances,
  useCreateAllowance,
  usePauseAllowance,
  useResumeAllowance,
} from "@/hooks/use-allowances";
import { useWallets } from "@/hooks/use-wallets";
import { useProfileStore } from "@/stores/profile-store";
import { toast } from "sonner";

const FREQUENCIES = [
  { value: "daily", label: "每日" },
  { value: "weekly", label: "每週" },
  { value: "biweekly", label: "每兩週" },
  { value: "monthly", label: "每月" },
];

export function AllowancesPage() {
  const { profile, family } = useProfileStore();
  const isGiver = profile?.role === "giver";
  const { data: allowances, isLoading } = useAllowances(family?.id);
  const { data: wallets } = useWallets(family?.id);
  const createAllowance = useCreateAllowance();
  const pauseAllowance = usePauseAllowance();
  const resumeAllowance = useResumeAllowance();

  const [open, setOpen] = useState(false);
  const [walletId, setWalletId] = useState("");
  const [amount, setAmount] = useState("");
  const [frequency, setFrequency] = useState("");

  const activeWallets = wallets?.filter((w) => w.status === "active") ?? [];

  async function handleCreate() {
    if (!family || !profile || !walletId || !amount || !frequency) return;
    try {
      await createAllowance.mutateAsync({
        family_id: family.id,
        giver_member_id: profile.id,
        receiver_member_id: profile.id, // Simplified — in production, select receiver
        wallet_id: Number(walletId),
        amount_cents: Math.round(Number(amount) * 100),
        frequency,
      });
      toast.success("津貼已建立");
      setOpen(false);
      setWalletId("");
      setAmount("");
      setFrequency("");
    } catch {
      toast.error("建立失敗");
    }
  }

  if (isLoading) return <LoadingSkeleton count={3} />;

  return (
    <div className="flex flex-col gap-4 p-4">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold">定期津貼</h1>
        {isGiver && (
          <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
              <Button size="sm">
                <Plus className="mr-1 size-4" />
                新增津貼
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>新增定期津貼</DialogTitle>
              </DialogHeader>
              <div className="flex flex-col gap-4">
                <div className="space-y-2">
                  <Label>錢包</Label>
                  <Select value={walletId} onValueChange={setWalletId}>
                    <SelectTrigger>
                      <SelectValue placeholder="選擇錢包" />
                    </SelectTrigger>
                    <SelectContent>
                      {activeWallets.map((w) => (
                        <SelectItem key={w.id} value={String(w.id)}>
                          {w.name}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                <div className="space-y-2">
                  <Label>金額（元）</Label>
                  <Input
                    type="number"
                    inputMode="numeric"
                    placeholder="0"
                    min="1"
                    value={amount}
                    onChange={(e) => setAmount(e.target.value)}
                  />
                </div>
                <div className="space-y-2">
                  <Label>頻率</Label>
                  <Select value={frequency} onValueChange={setFrequency}>
                    <SelectTrigger>
                      <SelectValue placeholder="選擇頻率" />
                    </SelectTrigger>
                    <SelectContent>
                      {FREQUENCIES.map((f) => (
                        <SelectItem key={f.value} value={f.value}>
                          {f.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                <Button
                  onClick={handleCreate}
                  disabled={!walletId || !amount || !frequency || createAllowance.isPending}
                >
                  建立
                </Button>
              </div>
            </DialogContent>
          </Dialog>
        )}
      </div>

      {!allowances || allowances.length === 0 ? (
        <EmptyState
          icon={<Calendar className="size-12" />}
          title="沒有津貼規則"
          description={isGiver ? "建立定期津貼自動發放零用金" : "等待 Giver 設定津貼規則"}
        />
      ) : (
        <div className="flex flex-col gap-3">
          {allowances.map((a) => (
            <AllowanceCard
              key={a.id}
              allowance={a}
              isGiver={isGiver}
              onPause={(id) => pauseAllowance.mutate(id)}
              onResume={(id) => resumeAllowance.mutate(id)}
            />
          ))}
        </div>
      )}
    </div>
  );
}

import { useState } from "react";
import { ListOrdered, Plus } from "lucide-react";
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
import { TransactionItem } from "@/components/transaction-item";
import { MonthlySummary } from "@/components/monthly-summary";
import { EmptyState } from "@/components/empty-state";
import { LoadingSkeleton } from "@/components/loading-skeleton";
import {
  useTransactions,
  useMonthlySummary,
  useCreateManualTransaction,
} from "@/hooks/use-transactions";
import { useWallets } from "@/hooks/use-wallets";
import { useProfileStore } from "@/stores/profile-store";
import { toast } from "sonner";

export function TransactionsPage() {
  const { profile, family } = useProfileStore();
  const isGiver = profile?.role === "giver";

  const now = new Date();
  const year = now.getFullYear();
  const month = now.getMonth() + 1;

  const { data: transactions, isLoading } = useTransactions(family?.id, { limit: 50 });
  const { data: summary } = useMonthlySummary(family?.id, year, month);
  const { data: wallets } = useWallets(family?.id);
  const createTransaction = useCreateManualTransaction();

  const [open, setOpen] = useState(false);
  const [walletId, setWalletId] = useState("");
  const [txType, setTxType] = useState("");
  const [amount, setAmount] = useState("");
  const [notes, setNotes] = useState("");

  const activeWallets = wallets?.filter((w) => w.status === "active") ?? [];

  async function handleCreate() {
    if (!family || !walletId || !txType || !amount) return;
    try {
      await createTransaction.mutateAsync({
        family_id: family.id,
        wallet_id: Number(walletId),
        transaction_type: txType,
        amount_cents: Math.round(Number(amount) * 100),
        notes: notes.trim() || undefined,
      });
      toast.success("交易已記錄");
      setOpen(false);
      setWalletId("");
      setTxType("");
      setAmount("");
      setNotes("");
    } catch {
      toast.error("記帳失敗");
    }
  }

  if (isLoading) return <LoadingSkeleton count={3} />;

  return (
    <div className="flex flex-col gap-4 p-4">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold">交易紀錄</h1>
        {isGiver && (
          <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
              <Button size="sm">
                <Plus className="mr-1 size-4" />
                手動記帳
              </Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>手動記帳</DialogTitle>
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
                  <Label>類型</Label>
                  <Select value={txType} onValueChange={setTxType}>
                    <SelectTrigger>
                      <SelectValue placeholder="選擇類型" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="credit">入帳</SelectItem>
                      <SelectItem value="debit">出帳</SelectItem>
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
                  <Label>備註</Label>
                  <Input
                    placeholder="備註（選填）"
                    value={notes}
                    onChange={(e) => setNotes(e.target.value)}
                  />
                </div>
                <Button
                  onClick={handleCreate}
                  disabled={!walletId || !txType || !amount || createTransaction.isPending}
                >
                  記錄
                </Button>
              </div>
            </DialogContent>
          </Dialog>
        )}
      </div>

      {summary && <MonthlySummary summary={summary} year={year} month={month} />}

      {!transactions || transactions.length === 0 ? (
        <EmptyState
          icon={<ListOrdered className="size-12" />}
          title="沒有交易紀錄"
          description="交易紀錄會在請款核准、津貼發放或手動記帳時自動產生"
        />
      ) : (
        <div className="flex flex-col divide-y">
          {transactions.map((tx) => (
            <TransactionItem key={tx.id} transaction={tx} />
          ))}
        </div>
      )}
    </div>
  );
}

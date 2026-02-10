import { useState } from "react";
import { useNavigate } from "react-router";
import { ArrowLeft } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useWallets } from "@/hooks/use-wallets";
import { useCreateRequest, useSubmitRequest } from "@/hooks/use-requests";
import { useProfileStore } from "@/stores/profile-store";
import { REQUEST_CATEGORIES } from "@/lib/types";
import { toast } from "sonner";

export function NewRequestPage() {
  const navigate = useNavigate();
  const { profile, family } = useProfileStore();
  const { data: wallets } = useWallets(family?.id);
  const createRequest = useCreateRequest();
  const submitRequest = useSubmitRequest();

  const [walletId, setWalletId] = useState<string>("");
  const [amount, setAmount] = useState("");
  const [category, setCategory] = useState<string>("");
  const [notes, setNotes] = useState("");
  const [submitting, setSubmitting] = useState(false);

  const activeWallets = wallets?.filter((w) => w.status === "active") ?? [];

  const isValid = walletId && amount && Number(amount) > 0 && Number(amount) <= 999999;

  async function handleSave(shouldSubmit: boolean) {
    if (!family || !profile || !walletId || !amount) return;
    setSubmitting(true);

    try {
      // We need the requester's member id. For now use profile id as member reference.
      // In production this would look up the family_member record for this profile.
      const request = await createRequest.mutateAsync({
        family_id: family.id,
        requester_member_id: profile.id,
        wallet_id: Number(walletId),
        amount_cents: Math.round(Number(amount) * 100),
        category: category || undefined,
        notes: notes.trim() || undefined,
      });

      if (shouldSubmit) {
        await submitRequest.mutateAsync(request.id);
        toast.success("請款已送出");
      } else {
        toast.success("草稿已儲存");
      }

      navigate("/requests");
    } catch (err: unknown) {
      const message =
        err && typeof err === "object" && "message" in err
          ? (err as { message: string }).message
          : "操作失敗";
      toast.error(message);
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="flex flex-col gap-4 p-4">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon" onClick={() => navigate("/requests")}>
          <ArrowLeft className="size-5" />
        </Button>
        <h1 className="text-xl font-bold">建立請款</h1>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>請款資訊</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <div className="space-y-2">
            <Label htmlFor="wallet">錢包</Label>
            <Select value={walletId} onValueChange={setWalletId}>
              <SelectTrigger id="wallet">
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
            <Label htmlFor="amount">金額（元）</Label>
            <Input
              id="amount"
              type="number"
              inputMode="numeric"
              placeholder="0"
              min="1"
              max="999999"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="category">分類</Label>
            <Select value={category} onValueChange={setCategory}>
              <SelectTrigger id="category">
                <SelectValue placeholder="選擇分類（可選）" />
              </SelectTrigger>
              <SelectContent>
                {REQUEST_CATEGORIES.map((c) => (
                  <SelectItem key={c.value} value={c.value}>
                    {c.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="space-y-2">
            <Label htmlFor="notes">備註</Label>
            <Input
              id="notes"
              placeholder="備註說明（選填，最多 500 字）"
              maxLength={500}
              value={notes}
              onChange={(e) => setNotes(e.target.value)}
            />
          </div>

          <div className="flex gap-3 pt-2">
            <Button
              variant="outline"
              className="flex-1"
              disabled={!isValid || submitting}
              onClick={() => handleSave(false)}
            >
              儲存草稿
            </Button>
            <Button
              className="flex-1"
              disabled={!isValid || submitting}
              onClick={() => handleSave(true)}
            >
              送出請款
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

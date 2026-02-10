import { useState } from "react";
import { useNavigate } from "react-router";
import { ArrowLeft, Users, UserPlus, UserMinus, Copy, Clock } from "lucide-react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from "@/components/ui/dialog";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { LoadingSkeleton } from "@/components/loading-skeleton";
import { EmptyState } from "@/components/empty-state";
import { useProfileStore } from "@/stores/profile-store";
import { formatDateTime } from "@/lib/format";
import type { FamilyMember } from "@/lib/types";

export function FamilyPage() {
  const navigate = useNavigate();
  const qc = useQueryClient();
  const family = useProfileStore((s) => s.family);
  const [pairingCode, setPairingCode] = useState<string | null>(null);
  const [pairingDialogOpen, setPairingDialogOpen] = useState(false);
  const [removeMember, setRemoveMember] = useState<FamilyMember | null>(null);

  const { data: members, isLoading } = useQuery({
    queryKey: ["family-members", family?.id],
    queryFn: () => invoke<FamilyMember[]>("get_family_members", { familyId: family!.id }),
    enabled: !!family,
  });

  const generateCode = useMutation({
    mutationFn: () => invoke<string>("generate_pairing_code"),
    onSuccess: (code) => {
      setPairingCode(code);
      setPairingDialogOpen(true);
    },
  });

  const removeMemberMut = useMutation({
    mutationFn: (memberId: number) => invoke("remove_family_member", { memberId }),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ["family-members"] });
      setRemoveMember(null);
    },
  });

  const copyCode = () => {
    if (pairingCode) {
      navigator.clipboard.writeText(pairingCode);
    }
  };

  return (
    <div className="p-4">
      <div className="mb-4 flex items-center gap-2">
        <Button variant="ghost" size="icon" onClick={() => navigate(-1)}>
          <ArrowLeft className="size-5" />
        </Button>
        <h1 className="text-lg font-bold">家庭管理</h1>
      </div>

      {/* Family info */}
      {family && (
        <div className="mb-4 rounded-lg border p-4">
          <p className="font-medium">{family.name}</p>
          <p className="text-sm text-muted-foreground">幣別：{family.currency}</p>
        </div>
      )}

      {/* Invite button */}
      <Button
        className="mb-4 w-full"
        onClick={() => generateCode.mutate()}
        disabled={generateCode.isPending}
      >
        <UserPlus className="mr-2 size-4" />
        {generateCode.isPending ? "產生中…" : "邀請成員"}
      </Button>

      {/* Members list */}
      {isLoading ? (
        <LoadingSkeleton />
      ) : !members || members.length === 0 ? (
        <EmptyState
          icon={<Users className="size-8" />}
          title="尚無成員"
          description="邀請家人加入"
        />
      ) : (
        <div className="space-y-2">
          <h2 className="text-sm font-semibold text-muted-foreground">成員（{members.length}）</h2>
          {members.map((member) => (
            <div key={member.id} className="flex items-center gap-3 rounded-lg border p-3">
              <div className="flex size-10 items-center justify-center rounded-full bg-muted">
                <Users className="size-5 text-muted-foreground" />
              </div>
              <div className="flex-1">
                <p className="text-sm font-medium">{member.profile_uuid}</p>
                <div className="flex items-center gap-2 text-xs text-muted-foreground">
                  <span>{member.family_role === "giver" ? "家長" : "孩子"}</span>
                  {member.joined_at && (
                    <>
                      <span>·</span>
                      <Clock className="size-3" />
                      <span>{formatDateTime(member.joined_at)}</span>
                    </>
                  )}
                </div>
              </div>
              {member.family_role !== "giver" && (
                <Button
                  variant="ghost"
                  size="icon"
                  className="text-destructive"
                  onClick={() => setRemoveMember(member)}
                >
                  <UserMinus className="size-4" />
                </Button>
              )}
            </div>
          ))}
        </div>
      )}

      {/* Pairing code dialog */}
      <Dialog open={pairingDialogOpen} onOpenChange={setPairingDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>配對碼</DialogTitle>
          </DialogHeader>
          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              請在孩子的裝置上輸入以下配對碼加入家庭：
            </p>
            <div className="flex items-center justify-center gap-2 rounded-lg bg-muted p-4">
              <span className="text-2xl font-mono font-bold tracking-widest">{pairingCode}</span>
              <Button variant="ghost" size="icon" onClick={copyCode}>
                <Copy className="size-4" />
              </Button>
            </div>
          </div>
          <DialogFooter>
            <Button onClick={() => setPairingDialogOpen(false)}>關閉</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Remove member confirm */}
      <AlertDialog open={!!removeMember} onOpenChange={(open) => !open && setRemoveMember(null)}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>移除成員？</AlertDialogTitle>
            <AlertDialogDescription>
              確定要移除此成員嗎？移除後對方將無法再同步資料。
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>取消</AlertDialogCancel>
            <AlertDialogAction
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
              onClick={() => removeMember && removeMemberMut.mutate(removeMember.id)}
            >
              確定移除
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}

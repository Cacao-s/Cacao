import { useState } from "react";
import { useNavigate, useParams } from "react-router";
import { ArrowLeft, CheckCircle2, XCircle, Ban, Send } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
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
import { Separator } from "@/components/ui/separator";
import { AmountDisplay } from "@/components/amount-display";
import { RequestStatusBadge } from "@/components/request-status-badge";
import { LoadingSkeleton } from "@/components/loading-skeleton";
import {
  useRequest,
  useApproveRequest,
  useRejectRequest,
  useCancelRequest,
  useSubmitRequest,
} from "@/hooks/use-requests";
import { useProfileStore } from "@/stores/profile-store";
import { formatDateTime } from "@/lib/format";
import { toast } from "sonner";

export function RequestDetailPage() {
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const { profile } = useProfileStore();
  const isGiver = profile?.role === "giver";

  const { data: request, isLoading } = useRequest(Number(id));
  const approveRequest = useApproveRequest();
  const rejectRequest = useRejectRequest();
  const cancelRequest = useCancelRequest();
  const submitRequest = useSubmitRequest();

  const [showApproveDialog, setShowApproveDialog] = useState(false);
  const [showRejectDialog, setShowRejectDialog] = useState(false);
  const [showCancelDialog, setShowCancelDialog] = useState(false);
  const [rejectionReason, setRejectionReason] = useState("");
  const [processing, setProcessing] = useState(false);

  if (isLoading || !request) {
    return <LoadingSkeleton count={2} />;
  }

  const isPending = request.status === "pending";
  const isDraft = request.status === "draft";
  const isDecided = request.status === "approved" || request.status === "rejected";

  async function handleApprove() {
    if (!profile) return;
    setProcessing(true);
    try {
      await approveRequest.mutateAsync({
        id: request!.id,
        approverMemberId: profile.id,
      });
      toast.success("請款已核准");
      navigate("/requests");
    } catch (err: unknown) {
      const message =
        err && typeof err === "object" && "message" in err
          ? (err as { message: string }).message
          : "核准失敗";
      toast.error(message);
    } finally {
      setProcessing(false);
      setShowApproveDialog(false);
    }
  }

  async function handleReject() {
    if (!profile || !rejectionReason.trim()) return;
    setProcessing(true);
    try {
      await rejectRequest.mutateAsync({
        id: request!.id,
        reason: rejectionReason.trim(),
        rejectorMemberId: profile.id,
      });
      toast.success("請款已駁回");
      navigate("/requests");
    } catch (err: unknown) {
      const message =
        err && typeof err === "object" && "message" in err
          ? (err as { message: string }).message
          : "駁回失敗";
      toast.error(message);
    } finally {
      setProcessing(false);
      setShowRejectDialog(false);
    }
  }

  async function handleCancel() {
    setProcessing(true);
    try {
      await cancelRequest.mutateAsync(request!.id);
      toast.success("請款已取消");
      navigate("/requests");
    } catch (err: unknown) {
      const message =
        err && typeof err === "object" && "message" in err
          ? (err as { message: string }).message
          : "取消失敗";
      toast.error(message);
    } finally {
      setProcessing(false);
      setShowCancelDialog(false);
    }
  }

  async function handleSubmit() {
    setProcessing(true);
    try {
      await submitRequest.mutateAsync(request!.id);
      toast.success("請款已送出");
      navigate("/requests");
    } catch (err: unknown) {
      const message =
        err && typeof err === "object" && "message" in err
          ? (err as { message: string }).message
          : "送出失敗";
      toast.error(message);
    } finally {
      setProcessing(false);
    }
  }

  const categoryLabels: Record<string, string> = {
    food: "食物",
    transport: "交通",
    education: "教育",
    entertainment: "娛樂",
    clothing: "服飾",
    health: "醫療",
    other: "其他",
  };

  return (
    <div className="flex flex-col gap-4 p-4">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon" onClick={() => navigate("/requests")}>
          <ArrowLeft className="size-5" />
        </Button>
        <h1 className="text-xl font-bold">請款詳情</h1>
      </div>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle>請款資訊</CardTitle>
          <RequestStatusBadge status={request.status} />
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <div className="text-center">
            <AmountDisplay cents={request.amount_cents} className="text-3xl font-bold" />
          </div>

          <Separator />

          <div className="grid grid-cols-2 gap-y-3 text-sm">
            <span className="text-muted-foreground">分類</span>
            <span>
              {request.category ? (categoryLabels[request.category] ?? request.category) : "未分類"}
            </span>

            <span className="text-muted-foreground">建立時間</span>
            <span>{formatDateTime(request.created_at)}</span>

            {request.decision_at && (
              <>
                <span className="text-muted-foreground">決定時間</span>
                <span>{formatDateTime(request.decision_at)}</span>
              </>
            )}
          </div>

          {request.notes && (
            <>
              <Separator />
              <div>
                <span className="text-sm text-muted-foreground">備註</span>
                <p className="mt-1 text-sm">{request.notes}</p>
              </div>
            </>
          )}

          {request.rejection_reason && (
            <>
              <Separator />
              <div>
                <span className="text-sm text-destructive">駁回原因</span>
                <p className="mt-1 text-sm">{request.rejection_reason}</p>
              </div>
            </>
          )}
        </CardContent>
      </Card>

      {/* Giver actions for pending requests */}
      {isGiver && isPending && (
        <div className="flex gap-3">
          <Button
            variant="outline"
            className="flex-1"
            disabled={processing}
            onClick={() => setShowRejectDialog(true)}
          >
            <XCircle className="mr-1 size-4" />
            駁回
          </Button>
          <Button
            className="flex-1"
            disabled={processing}
            onClick={() => setShowApproveDialog(true)}
          >
            <CheckCircle2 className="mr-1 size-4" />
            核准
          </Button>
        </div>
      )}

      {/* Baby actions for draft */}
      {!isGiver && isDraft && (
        <div className="flex gap-3">
          <Button
            variant="outline"
            className="flex-1"
            disabled={processing}
            onClick={() => setShowCancelDialog(true)}
          >
            <Ban className="mr-1 size-4" />
            取消
          </Button>
          <Button className="flex-1" disabled={processing} onClick={handleSubmit}>
            <Send className="mr-1 size-4" />
            送出
          </Button>
        </div>
      )}

      {/* Baby actions for pending */}
      {!isGiver && isPending && (
        <Button variant="outline" disabled={processing} onClick={() => setShowCancelDialog(true)}>
          <Ban className="mr-1 size-4" />
          取消請款
        </Button>
      )}

      {/* Decided — read only, no actions */}
      {isDecided && <p className="text-center text-sm text-muted-foreground">此請款已結案</p>}

      {/* Approve dialog */}
      <AlertDialog open={showApproveDialog} onOpenChange={setShowApproveDialog}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>確認核准</AlertDialogTitle>
            <AlertDialogDescription>
              核准此請款將從錢包扣除{" "}
              <AmountDisplay cents={request.amount_cents} className="font-semibold" />
              ，此操作無法撤銷。
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel disabled={processing}>取消</AlertDialogCancel>
            <AlertDialogAction disabled={processing} onClick={handleApprove}>
              確認核准
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      {/* Reject dialog */}
      <AlertDialog open={showRejectDialog} onOpenChange={setShowRejectDialog}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>駁回請款</AlertDialogTitle>
            <AlertDialogDescription>請輸入駁回原因（必填）</AlertDialogDescription>
          </AlertDialogHeader>
          <div className="px-6 pb-2">
            <Label htmlFor="rejection-reason" className="sr-only">
              駁回原因
            </Label>
            <Input
              id="rejection-reason"
              placeholder="請說明駁回原因"
              value={rejectionReason}
              onChange={(e) => setRejectionReason(e.target.value)}
            />
          </div>
          <AlertDialogFooter>
            <AlertDialogCancel disabled={processing}>取消</AlertDialogCancel>
            <AlertDialogAction
              variant="destructive"
              disabled={processing || !rejectionReason.trim()}
              onClick={handleReject}
            >
              確認駁回
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      {/* Cancel dialog */}
      <AlertDialog open={showCancelDialog} onOpenChange={setShowCancelDialog}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>取消請款</AlertDialogTitle>
            <AlertDialogDescription>確定要取消此請款嗎？此操作無法撤銷。</AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel disabled={processing}>返回</AlertDialogCancel>
            <AlertDialogAction variant="destructive" disabled={processing} onClick={handleCancel}>
              確認取消
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}

import { useState } from "react";
import { useNavigate } from "react-router";
import { Plus, Receipt } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Badge } from "@/components/ui/badge";
import { RequestCard } from "@/components/request-card";
import { EmptyState } from "@/components/empty-state";
import { LoadingSkeleton } from "@/components/loading-skeleton";
import { useRequests } from "@/hooks/use-requests";
import { useProfileStore } from "@/stores/profile-store";

export function RequestsPage() {
  const navigate = useNavigate();
  const { profile, family } = useProfileStore();
  const [tab, setTab] = useState("all");
  const isGiver = profile?.role === "giver";

  const statusFilter = tab === "all" ? undefined : tab;
  const { data: requests, isLoading, error } = useRequests(family?.id, statusFilter);

  const pendingCount = requests?.filter((r) => r.status === "pending").length ?? 0;

  if (isLoading) return <LoadingSkeleton count={3} />;

  if (error) {
    return <div className="p-4 text-center text-destructive">載入失敗，請稍後再試</div>;
  }

  return (
    <div className="flex flex-col gap-4 p-4">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold">請款</h1>
        {!isGiver && (
          <Button size="sm" onClick={() => navigate("/requests/new")}>
            <Plus className="mr-1 size-4" />
            建立請款
          </Button>
        )}
      </div>

      <Tabs value={tab} onValueChange={setTab}>
        <TabsList className="w-full">
          <TabsTrigger value="all" className="flex-1">
            全部
          </TabsTrigger>
          <TabsTrigger value="pending" className="flex-1 gap-1">
            待審核
            {pendingCount > 0 && tab !== "pending" && (
              <Badge variant="destructive" className="ml-1 px-1.5 py-0 text-[10px]">
                {pendingCount}
              </Badge>
            )}
          </TabsTrigger>
          <TabsTrigger value="approved" className="flex-1">
            已核准
          </TabsTrigger>
          <TabsTrigger value="rejected" className="flex-1">
            已駁回
          </TabsTrigger>
        </TabsList>

        <TabsContent value={tab} className="mt-4">
          {!requests || requests.length === 0 ? (
            <EmptyState
              icon={<Receipt className="size-12" />}
              title="沒有請款紀錄"
              description={
                isGiver ? "等待 Baby 提交請款申請" : "點擊「建立請款」提交你的第一筆請款"
              }
              action={
                !isGiver ? (
                  <Button onClick={() => navigate("/requests/new")}>建立請款</Button>
                ) : undefined
              }
            />
          ) : (
            <div className="flex flex-col gap-3">
              {requests.map((request) => (
                <RequestCard key={request.id} request={request} />
              ))}
            </div>
          )}
        </TabsContent>
      </Tabs>
    </div>
  );
}

import { useState } from "react";
import { useNavigate } from "react-router";
import {
  User,
  Shield,
  Lock,
  FileDown,
  Users,
  RotateCcw,
  ChevronRight,
  Globe,
  Palette,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
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
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Separator } from "@/components/ui/separator";
import { useProfileStore } from "@/stores/profile-store";
import { useHasPin, useSetPin, useRemovePin, useVerifyPin } from "@/hooks/use-auth";
import { useExportCsv } from "@/hooks/use-export";
import { useUiStore } from "@/stores/ui-store";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";

function SettingsItem({
  icon: Icon,
  label,
  description,
  onClick,
}: {
  icon: typeof User;
  label: string;
  description?: string;
  onClick?: () => void;
}) {
  return (
    <button
      className="flex w-full items-center gap-3 rounded-lg p-3 text-left transition-colors hover:bg-accent"
      onClick={onClick}
    >
      <Icon className="size-5 shrink-0 text-muted-foreground" />
      <div className="flex-1">
        <p className="text-sm font-medium">{label}</p>
        {description && <p className="text-xs text-muted-foreground">{description}</p>}
      </div>
      <ChevronRight className="size-4 text-muted-foreground" />
    </button>
  );
}

const localeOptions = [
  { value: "zh-TW" as const, label: "繁體中文" },
  { value: "en" as const, label: "English" },
];

const themeOptions = [
  { value: "light" as const, label: "淺色" },
  { value: "dark" as const, label: "深色" },
  { value: "high-contrast" as const, label: "高對比" },
];

export function SettingsPage() {
  const navigate = useNavigate();
  const { t } = useTranslation();
  const profile = useProfileStore((s) => s.profile);
  const family = useProfileStore((s) => s.family);
  const isGiver = profile?.role === "giver";

  const { locale, theme, setLocale, setTheme } = useUiStore();
  const { data: hasPin } = useHasPin();
  const setPin = useSetPin();
  const removePin = useRemovePin();
  const verifyPin = useVerifyPin();
  const exportCsv = useExportCsv();

  const [pinDialogOpen, setPinDialogOpen] = useState(false);
  const [pinValue, setPinValue] = useState("");
  const [pinConfirm, setPinConfirm] = useState("");
  const [pinError, setPinError] = useState("");
  const [pinStep, setPinStep] = useState<"current" | "new" | "confirm">("new");

  const [exportDialogOpen, setExportDialogOpen] = useState(false);
  const [exportFrom, setExportFrom] = useState("");
  const [exportTo, setExportTo] = useState("");

  const handlePinOpen = () => {
    setPinValue("");
    setPinConfirm("");
    setPinError("");
    setPinStep(hasPin ? "current" : "new");
    setPinDialogOpen(true);
  };

  const handlePinSubmit = () => {
    if (pinStep === "current") {
      verifyPin.mutate(pinValue, {
        onSuccess: (valid) => {
          if (valid) {
            setPinValue("");
            setPinStep("new");
          } else {
            setPinError("PIN 碼錯誤");
          }
        },
      });
      return;
    }

    if (pinStep === "new") {
      if (pinValue.length < 4 || pinValue.length > 6) {
        setPinError("PIN 碼需為 4~6 位數字");
        return;
      }
      setPinConfirm("");
      setPinStep("confirm");
      setPinError("");
      return;
    }

    if (pinStep === "confirm") {
      if (pinConfirm !== pinValue) {
        setPinError("PIN 碼不一致");
        return;
      }
      setPin.mutate(pinValue, {
        onSuccess: () => setPinDialogOpen(false),
      });
    }
  };

  const handleExport = () => {
    if (!family) return;
    exportCsv.mutate(
      {
        familyId: family.id,
        dateFrom: exportFrom || undefined,
        dateTo: exportTo || undefined,
      },
      {
        onSuccess: (csv) => {
          const blob = new Blob([csv], { type: "text/csv;charset=utf-8;" });
          const url = URL.createObjectURL(blob);
          const a = document.createElement("a");
          a.href = url;
          a.download = `cacao-transactions-${new Date().toISOString().slice(0, 10)}.csv`;
          a.click();
          URL.revokeObjectURL(url);
          setExportDialogOpen(false);
        },
      },
    );
  };

  const handleResetDevice = async () => {
    try {
      await invoke("reset_device");
      window.location.href = "/setup";
    } catch {
      // error handling
    }
  };

  return (
    <div className="p-4">
      <h1 className="mb-4 text-lg font-bold">設定</h1>

      {/* Profile info */}
      <div className="mb-4 rounded-lg border p-4">
        <div className="flex items-center gap-3">
          <div className="flex size-12 items-center justify-center rounded-full bg-primary/10">
            <User className="size-6 text-primary" />
          </div>
          <div>
            <p className="font-medium">{profile?.display_name}</p>
            <p className="text-sm text-muted-foreground">
              {isGiver ? "家長（Giver）" : "孩子（Baby）"}
            </p>
          </div>
        </div>
      </div>

      <div className="space-y-1">
        {/* Family management (Giver) */}
        {isGiver && (
          <SettingsItem
            icon={Users}
            label="家庭管理"
            description={family?.name}
            onClick={() => navigate("/family")}
          />
        )}

        <Separator />

        {/* Language */}
        <div className="flex w-full items-center gap-3 rounded-lg p-3">
          <Globe className="size-5 shrink-0 text-muted-foreground" />
          <div className="flex-1">
            <p className="text-sm font-medium">{t("settings.language")}</p>
          </div>
          <div className="flex gap-1">
            {localeOptions.map((opt) => (
              <Button
                key={opt.value}
                variant={locale === opt.value ? "default" : "outline"}
                size="sm"
                className="text-xs"
                onClick={() => setLocale(opt.value)}
              >
                {opt.label}
              </Button>
            ))}
          </div>
        </div>

        <Separator />

        {/* Theme */}
        <div className="flex w-full items-center gap-3 rounded-lg p-3">
          <Palette className="size-5 shrink-0 text-muted-foreground" />
          <div className="flex-1">
            <p className="text-sm font-medium">{t("settings.theme")}</p>
          </div>
          <div className="flex gap-1">
            {themeOptions.map((opt) => (
              <Button
                key={opt.value}
                variant={theme === opt.value ? "default" : "outline"}
                size="sm"
                className="text-xs"
                onClick={() => setTheme(opt.value)}
              >
                {opt.label}
              </Button>
            ))}
          </div>
        </div>

        <Separator />

        {/* PIN lock */}
        <SettingsItem
          icon={Lock}
          label="App 鎖定"
          description={hasPin ? "已設定 PIN" : "未設定"}
          onClick={handlePinOpen}
        />

        <Separator />

        {/* Export */}
        {isGiver && (
          <>
            <SettingsItem
              icon={FileDown}
              label="匯出交易紀錄"
              description="CSV 格式"
              onClick={() => setExportDialogOpen(true)}
            />
            <Separator />
          </>
        )}

        {/* Audit log (Giver) */}
        {isGiver && (
          <>
            <SettingsItem
              icon={Shield}
              label="審計日誌"
              description="查看所有操作紀錄"
              onClick={() => navigate("/settings/audit-log")}
            />
            <Separator />
          </>
        )}

        {/* Reset */}
        <AlertDialog>
          <AlertDialogTrigger asChild>
            <button className="flex w-full items-center gap-3 rounded-lg p-3 text-left transition-colors hover:bg-accent">
              <RotateCcw className="size-5 shrink-0 text-destructive" />
              <div className="flex-1">
                <p className="text-sm font-medium text-destructive">重設裝置</p>
                <p className="text-xs text-muted-foreground">清除所有本地資料</p>
              </div>
            </button>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>確定要重設裝置？</AlertDialogTitle>
              <AlertDialogDescription>
                此操作將清除所有本地資料，包括錢包、交易紀錄及設定。此操作無法復原。
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>取消</AlertDialogCancel>
              <AlertDialogAction
                className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                onClick={handleResetDevice}
              >
                確定重設
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </div>

      {/* PIN Dialog */}
      <Dialog open={pinDialogOpen} onOpenChange={setPinDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {pinStep === "current"
                ? "輸入目前 PIN"
                : pinStep === "new"
                  ? "設定新 PIN"
                  : "確認 PIN"}
            </DialogTitle>
          </DialogHeader>
          <div className="space-y-3">
            <div>
              <Label>{pinStep === "confirm" ? "再次輸入 PIN" : "PIN 碼（4~6 位數字）"}</Label>
              <Input
                type="password"
                inputMode="numeric"
                maxLength={6}
                value={pinStep === "confirm" ? pinConfirm : pinValue}
                onChange={(e) => {
                  const v = e.target.value.replace(/\D/g, "");
                  if (pinStep === "confirm") setPinConfirm(v);
                  else setPinValue(v);
                  setPinError("");
                }}
                placeholder="••••"
              />
            </div>
            {pinError && <p className="text-sm text-destructive">{pinError}</p>}
          </div>
          <DialogFooter className="flex-row gap-2">
            {hasPin && pinStep === "new" && (
              <Button
                variant="destructive"
                onClick={() => {
                  removePin.mutate(undefined, {
                    onSuccess: () => setPinDialogOpen(false),
                  });
                }}
              >
                移除 PIN
              </Button>
            )}
            <Button onClick={handlePinSubmit}>{pinStep === "confirm" ? "確定" : "下一步"}</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Export Dialog */}
      <Dialog open={exportDialogOpen} onOpenChange={setExportDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>匯出交易紀錄</DialogTitle>
          </DialogHeader>
          <div className="space-y-3">
            <div>
              <Label>開始日期（選填）</Label>
              <Input
                type="date"
                value={exportFrom}
                onChange={(e) => setExportFrom(e.target.value)}
              />
            </div>
            <div>
              <Label>結束日期（選填）</Label>
              <Input type="date" value={exportTo} onChange={(e) => setExportTo(e.target.value)} />
            </div>
          </div>
          <DialogFooter>
            <Button onClick={handleExport} disabled={exportCsv.isPending}>
              {exportCsv.isPending ? "匯出中…" : "匯出 CSV"}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
}

import { useState, useCallback } from "react";
import { useNavigate } from "react-router";
import { useMutation } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { ShieldCheck, Baby } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { cn } from "@/lib/utils";
import { useProfileStore } from "@/stores/profile-store";
import type { Profile, Family } from "@/lib/types";

type Role = "giver" | "baby";

/** Total steps: Giver = 4 steps, Baby = 3 steps */
function getTotalSteps(role: Role | null): number {
  if (role === "giver") return 4;
  if (role === "baby") return 3;
  return 3; // default before role is selected
}

function StepIndicator({ current, total }: { current: number; total: number }) {
  return (
    <div className="flex items-center justify-center gap-2">
      {Array.from({ length: total }, (_, i) => (
        <div
          key={i}
          className={cn(
            "h-2 w-2 rounded-full transition-colors",
            i + 1 === current ? "bg-primary" : "bg-muted",
          )}
        />
      ))}
    </div>
  );
}

export function SetupPage() {
  const navigate = useNavigate();
  const { setProfile, setFamily } = useProfileStore();

  const [step, setStep] = useState(1);
  const [displayName, setDisplayName] = useState("");
  const [role, setRole] = useState<Role | null>(null);
  const [familyName, setFamilyName] = useState("");
  const [pairingCode, setPairingCode] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const totalSteps = getTotalSteps(role);

  // Setup device mutation
  const setupDevice = useMutation({
    mutationFn: (params: { displayName: string; role: string }) =>
      invoke<Profile>("setup_device", {
        displayName: params.displayName,
        role: params.role,
      }),
  });

  // Create family mutation
  const createFamily = useMutation({
    mutationFn: (params: { name: string }) =>
      invoke<Family>("create_family", { name: params.name }),
  });

  // Generate pairing code mutation
  const generateCode = useMutation({
    mutationFn: (params: { familyId: number }) =>
      invoke<string>("generate_pairing_code", { familyId: params.familyId }),
  });

  const isNameValid = displayName.trim().length >= 1 && displayName.trim().length <= 50;
  const isFamilyNameValid = familyName.trim().length >= 1 && familyName.trim().length <= 50;

  const handleRoleSelect = (selectedRole: Role) => {
    setRole(selectedRole);
    setStep(3);
  };

  // Giver flow: setup device -> create family -> generate code -> show code
  const handleGiverSetupFamily = useCallback(async () => {
    setError(null);
    try {
      const profile = await setupDevice.mutateAsync({
        displayName: displayName.trim(),
        role: "giver",
      });
      setProfile(profile);

      const family = await createFamily.mutateAsync({
        name: familyName.trim(),
      });
      setFamily(family);

      const code = await generateCode.mutateAsync({ familyId: family.id });
      setPairingCode(code);
      setStep(4);
    } catch (err: unknown) {
      const e = err as Record<string, unknown>;
      setError(typeof e?.message === "string" ? e.message : String(err));
    }
  }, [displayName, familyName, setupDevice, createFamily, generateCode, setProfile, setFamily]);

  // Baby flow: setup device -> navigate to pairing
  const handleBabySetup = useCallback(async () => {
    setError(null);
    try {
      const profile = await setupDevice.mutateAsync({
        displayName: displayName.trim(),
        role: "baby",
      });
      setProfile(profile);
      navigate("/pairing?mode=baby");
    } catch (err: unknown) {
      const e = err as Record<string, unknown>;
      setError(typeof e?.message === "string" ? e.message : String(err));
    }
  }, [displayName, setupDevice, setProfile, navigate]);

  const handleFinish = () => {
    navigate("/", { replace: true });
  };

  const isLoading = setupDevice.isPending || createFamily.isPending || generateCode.isPending;

  return (
    <div className="flex min-h-screen flex-col items-center justify-center px-6 py-8 max-w-md mx-auto">
      <div className="w-full space-y-8">
        {/* Header */}
        <div className="text-center space-y-2">
          <h1 className="text-2xl font-bold">
            {step === 1 && "歡迎使用 Cacao"}
            {step === 2 && "選擇角色"}
            {step === 3 && role === "giver" && "建立家庭"}
            {step === 3 && role === "baby" && "設定中..."}
            {step === 4 && "配對碼"}
          </h1>
          <p className="text-sm text-muted-foreground">
            {step === 1 && "請輸入您的顯示名稱"}
            {step === 2 && "您是家長還是孩子？"}
            {step === 3 && role === "giver" && "為您的家庭取一個名稱"}
            {step === 4 && "分享此配對碼給家庭成員"}
          </p>
        </div>

        {/* Step indicator */}
        <StepIndicator current={step} total={totalSteps} />

        {/* Step 1: Display name */}
        {step === 1 && (
          <div className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="display-name">顯示名稱</Label>
              <Input
                id="display-name"
                placeholder="請輸入名稱"
                value={displayName}
                onChange={(e) => setDisplayName(e.target.value)}
                maxLength={50}
                autoFocus
              />
              <p className="text-xs text-muted-foreground">{displayName.trim().length}/50 字元</p>
            </div>
            <Button className="w-full" size="lg" disabled={!isNameValid} onClick={() => setStep(2)}>
              下一步
            </Button>
          </div>
        )}

        {/* Step 2: Role selection */}
        {step === 2 && (
          <div className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <Card
                className={cn(
                  "cursor-pointer transition-all hover:border-primary",
                  role === "giver" && "border-primary ring-2 ring-primary/20",
                )}
                onClick={() => handleRoleSelect("giver")}
              >
                <CardHeader className="items-center text-center">
                  <ShieldCheck className="size-12 text-primary" />
                  <CardTitle className="text-base">家長</CardTitle>
                </CardHeader>
                <CardContent>
                  <CardDescription className="text-center text-xs">
                    管理家庭、錢包與零用錢
                  </CardDescription>
                </CardContent>
              </Card>

              <Card
                className={cn(
                  "cursor-pointer transition-all hover:border-primary",
                  role === "baby" && "border-primary ring-2 ring-primary/20",
                )}
                onClick={() => handleRoleSelect("baby")}
              >
                <CardHeader className="items-center text-center">
                  <Baby className="size-12 text-primary" />
                  <CardTitle className="text-base">孩子</CardTitle>
                </CardHeader>
                <CardContent>
                  <CardDescription className="text-center text-xs">
                    查看錢包、提出請求
                  </CardDescription>
                </CardContent>
              </Card>
            </div>

            <Button variant="ghost" className="w-full" onClick={() => setStep(1)}>
              上一步
            </Button>
          </div>
        )}

        {/* Step 3 (Giver): Family name */}
        {step === 3 && role === "giver" && (
          <div className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="family-name">家庭名稱</Label>
              <Input
                id="family-name"
                placeholder="例如：王家"
                value={familyName}
                onChange={(e) => setFamilyName(e.target.value)}
                maxLength={50}
                autoFocus
              />
              <p className="text-xs text-muted-foreground">{familyName.trim().length}/50 字元</p>
            </div>

            {error && <p className="text-sm text-destructive text-center">{error}</p>}

            <Button
              className="w-full"
              size="lg"
              disabled={!isFamilyNameValid || isLoading}
              onClick={handleGiverSetupFamily}
            >
              {isLoading ? "建立中..." : "建立家庭"}
            </Button>

            <Button
              variant="ghost"
              className="w-full"
              disabled={isLoading}
              onClick={() => setStep(2)}
            >
              上一步
            </Button>
          </div>
        )}

        {/* Step 3 (Baby): triggers navigation via handleBabySetup */}
        {step === 3 && role === "baby" && (
          <div className="space-y-4">
            {error && <p className="text-sm text-destructive text-center">{error}</p>}

            <Button className="w-full" size="lg" disabled={isLoading} onClick={handleBabySetup}>
              {isLoading ? "設定中..." : "開始配對"}
            </Button>

            <Button
              variant="ghost"
              className="w-full"
              disabled={isLoading}
              onClick={() => setStep(2)}
            >
              上一步
            </Button>
          </div>
        )}

        {/* Step 4 (Giver): Show pairing code */}
        {step === 4 && pairingCode && (
          <div className="space-y-6">
            <Card>
              <CardContent className="flex flex-col items-center gap-4 py-8">
                <p className="text-sm text-muted-foreground">配對碼</p>
                <p className="text-4xl font-mono font-bold tracking-[0.3em]">{pairingCode}</p>
                <p className="text-xs text-muted-foreground text-center">
                  請在孩子的裝置上輸入此配對碼以加入家庭
                </p>
              </CardContent>
            </Card>

            <Button className="w-full" size="lg" onClick={handleFinish}>
              完成設定
            </Button>

            <Button variant="ghost" className="w-full" onClick={handleFinish}>
              稍後配對
            </Button>
          </div>
        )}
      </div>
    </div>
  );
}

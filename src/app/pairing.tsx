import { useState, useEffect, useRef, useCallback } from "react";
import { useNavigate, useSearchParams } from "react-router";
import { useMutation, useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { RefreshCw } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent } from "@/components/ui/card";
import { useProfileStore } from "@/stores/profile-store";
import type { Family } from "@/lib/types";

const CODE_LENGTH = 6;
const COUNTDOWN_SECONDS = 5 * 60; // 5 minutes

// ─── Giver Mode ──────────────────────────────────────────────────────────────

function GiverPairing() {
  const { family } = useProfileStore();
  const [countdown, setCountdown] = useState(COUNTDOWN_SECONDS);
  const [refreshKey, setRefreshKey] = useState(0);
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const {
    data: code,
    isLoading,
    refetch,
  } = useQuery({
    queryKey: ["pairing-code", family?.id, refreshKey],
    queryFn: () => {
      if (!family) throw new Error("No family found");
      return invoke<string>("generate_pairing_code", {
        familyId: family.id,
      });
    },
    enabled: !!family,
  });

  // Countdown timer
  useEffect(() => {
    timerRef.current = setInterval(() => {
      setCountdown((prev) => {
        if (prev <= 1) {
          if (timerRef.current) clearInterval(timerRef.current);
          return 0;
        }
        return prev - 1;
      });
    }, 1000);

    return () => {
      if (timerRef.current) clearInterval(timerRef.current);
    };
  }, [refreshKey]);

  const handleRefresh = useCallback(() => {
    setCountdown(COUNTDOWN_SECONDS);
    setRefreshKey((k) => k + 1);
    refetch();
  }, [refetch]);

  const minutes = Math.floor(countdown / 60);
  const seconds = countdown % 60;

  if (!family) {
    return (
      <div className="flex min-h-screen items-center justify-center px-6">
        <p className="text-muted-foreground">找不到家庭資料，請先完成設定。</p>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen flex-col items-center justify-center px-6 py-8 max-w-md mx-auto">
      <div className="w-full space-y-8">
        <div className="text-center space-y-2">
          <h1 className="text-2xl font-bold">家庭配對碼</h1>
          <p className="text-sm text-muted-foreground">請在孩子的裝置上輸入此配對碼</p>
        </div>

        <Card>
          <CardContent className="flex flex-col items-center gap-4 py-8">
            {isLoading ? (
              <div className="animate-spin h-8 w-8 border-4 border-primary border-t-transparent rounded-full" />
            ) : (
              <>
                <p className="text-4xl font-mono font-bold tracking-[0.3em]">{code}</p>
                <p className="text-sm text-muted-foreground tabular-nums">
                  {countdown > 0
                    ? `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`
                    : "已過期"}
                </p>
              </>
            )}
          </CardContent>
        </Card>

        <Button variant="outline" className="w-full" onClick={handleRefresh} disabled={isLoading}>
          <RefreshCw className="size-4" />
          重新產生配對碼
        </Button>
      </div>
    </div>
  );
}

// ─── Baby Mode ───────────────────────────────────────────────────────────────

function BabyPairing() {
  const navigate = useNavigate();
  const { setFamily } = useProfileStore();
  const [digits, setDigits] = useState<string[]>(Array.from({ length: CODE_LENGTH }, () => ""));
  const [error, setError] = useState<string | null>(null);
  const inputRefs = useRef<(HTMLInputElement | null)[]>([]);

  const joinFamily = useMutation({
    mutationFn: (code: string) => invoke<Family>("join_family_with_code", { code }),
    onSuccess: (family) => {
      setFamily(family);
      navigate("/", { replace: true });
    },
    onError: (err: unknown) => {
      const e = err as Record<string, unknown>;
      setError(typeof e?.message === "string" ? e.message : String(err));
    },
  });

  const handleDigitChange = (index: number, value: string) => {
    // Only allow single digit
    const digit = value.replace(/\D/g, "").slice(-1);
    const newDigits = [...digits];
    newDigits[index] = digit;
    setDigits(newDigits);
    setError(null);

    // Auto-focus next input
    if (digit && index < CODE_LENGTH - 1) {
      inputRefs.current[index + 1]?.focus();
    }
  };

  const handleKeyDown = (index: number, e: React.KeyboardEvent) => {
    if (e.key === "Backspace" && !digits[index] && index > 0) {
      inputRefs.current[index - 1]?.focus();
    }
  };

  const handlePaste = (e: React.ClipboardEvent) => {
    e.preventDefault();
    const pasted = e.clipboardData.getData("text").replace(/\D/g, "").slice(0, CODE_LENGTH);
    if (pasted.length === 0) return;

    const newDigits = [...digits];
    for (let i = 0; i < pasted.length; i++) {
      newDigits[i] = pasted[i];
    }
    setDigits(newDigits);
    setError(null);

    // Focus the next empty input or the last one
    const nextEmpty = newDigits.findIndex((d) => !d);
    const focusIndex = nextEmpty === -1 ? CODE_LENGTH - 1 : nextEmpty;
    inputRefs.current[focusIndex]?.focus();
  };

  const code = digits.join("");
  const isComplete = code.length === CODE_LENGTH && digits.every((d) => d !== "");

  const handleSubmit = () => {
    if (!isComplete) return;
    joinFamily.mutate(code);
  };

  return (
    <div className="flex min-h-screen flex-col items-center justify-center px-6 py-8 max-w-md mx-auto">
      <div className="w-full space-y-8">
        <div className="text-center space-y-2">
          <h1 className="text-2xl font-bold">輸入配對碼</h1>
          <p className="text-sm text-muted-foreground">請輸入家長裝置上顯示的 6 位配對碼</p>
        </div>

        {/* Digit inputs */}
        <div className="flex justify-center gap-2">
          {digits.map((digit, i) => (
            <Input
              key={i}
              ref={(el) => {
                inputRefs.current[i] = el;
              }}
              type="text"
              inputMode="numeric"
              maxLength={1}
              value={digit}
              onChange={(e) => handleDigitChange(i, e.target.value)}
              onKeyDown={(e) => handleKeyDown(i, e)}
              onPaste={i === 0 ? handlePaste : undefined}
              className="w-12 h-14 text-center text-2xl font-mono font-bold"
              autoFocus={i === 0}
            />
          ))}
        </div>

        {error && <p className="text-sm text-destructive text-center">{error}</p>}

        <Button
          className="w-full"
          size="lg"
          disabled={!isComplete || joinFamily.isPending}
          onClick={handleSubmit}
        >
          {joinFamily.isPending ? "加入中..." : "加入家庭"}
        </Button>
      </div>
    </div>
  );
}

// ─── Main Component ──────────────────────────────────────────────────────────

export function PairingPage() {
  const [searchParams] = useSearchParams();
  const mode = searchParams.get("mode");

  if (mode === "baby") {
    return <BabyPairing />;
  }

  return <GiverPairing />;
}

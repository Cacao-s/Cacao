import { useState } from "react";
import { Lock, Delete, Fingerprint } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useVerifyPin } from "@/hooks/use-auth";

interface LockScreenProps {
  onUnlock: () => void;
}

export function LockScreen({ onUnlock }: LockScreenProps) {
  const [pin, setPin] = useState("");
  const [error, setError] = useState("");
  const verifyPin = useVerifyPin();

  const handleDigit = (digit: string) => {
    if (pin.length >= 6) return;
    const next = pin + digit;
    setPin(next);
    setError("");

    if (next.length >= 4) {
      verifyPin.mutate(next, {
        onSuccess: (valid) => {
          if (valid) {
            onUnlock();
          } else {
            setError("PIN 錯誤，請重試");
            setPin("");
          }
        },
        onError: () => {
          setError("驗證失敗");
          setPin("");
        },
      });
    }
  };

  const handleDelete = () => {
    setPin((prev) => prev.slice(0, -1));
    setError("");
  };

  return (
    <div className="flex h-screen flex-col items-center justify-center gap-8 bg-background p-8">
      <div className="flex flex-col items-center gap-2">
        <Lock className="size-10 text-primary" />
        <h1 className="text-xl font-bold">Cacao</h1>
        <p className="text-sm text-muted-foreground">請輸入 PIN 碼解鎖</p>
      </div>

      {/* PIN dots */}
      <div className="flex gap-3">
        {Array.from({ length: 6 }).map((_, i) => (
          <div
            key={i}
            className={`size-3 rounded-full transition-colors ${
              i < pin.length ? "bg-primary" : "bg-muted"
            }`}
          />
        ))}
      </div>

      {error && <p className="text-sm text-destructive">{error}</p>}

      {/* Number pad */}
      <div className="grid grid-cols-3 gap-4">
        {["1", "2", "3", "4", "5", "6", "7", "8", "9"].map((digit) => (
          <Button
            key={digit}
            variant="outline"
            className="size-16 text-xl font-medium"
            onClick={() => handleDigit(digit)}
          >
            {digit}
          </Button>
        ))}
        <Button
          variant="ghost"
          className="size-16"
          onClick={() => {
            /* biometric placeholder */
          }}
        >
          <Fingerprint className="size-6" />
        </Button>
        <Button
          variant="outline"
          className="size-16 text-xl font-medium"
          onClick={() => handleDigit("0")}
        >
          0
        </Button>
        <Button
          variant="ghost"
          className="size-16"
          onClick={handleDelete}
          disabled={pin.length === 0}
        >
          <Delete className="size-6" />
        </Button>
      </div>
    </div>
  );
}

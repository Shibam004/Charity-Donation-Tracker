"use client";

import { cn } from "@/lib/utils";
import { useMemo } from "react";

interface MeteorsProps {
  number?: number;
  className?: string;
}

export function Meteors({ number = 15, className }: MeteorsProps) {
  // Use deterministic values based on index to avoid hydration mismatch
  const meteors = useMemo(
    () =>
      Array.from({ length: number }, (_, i) => ({
        id: i,
        left: `${(i * 7 + 13) % 100}%`,
        delay: `${((i * 3 + 7) % 50) / 10}s`,
        duration: `${((i * 5 + 11) % 30) / 10 + 2}s`,
      })),
    [number]
  );

  return (
    <>
      {meteors.map((m) => (
        <span
          key={m.id}
          className={cn(
            "pointer-events-none absolute top-0 left-1/2 h-0.5 w-0.5 rotate-[215deg] animate-meteor rounded-full bg-white/80 shadow-[0_0_0_1px_#ffffff10]",
            "before:content-[''] before:absolute before:top-1/2 before:right-0 before:h-px before:w-[50px] before:-translate-y-1/2 before:bg-gradient-to-r before:from-[#7c6cf099] before:to-transparent",
            className
          )}
          style={{
            left: m.left,
            animationDelay: m.delay,
            animationDuration: m.duration,
          }}
        />
      ))}
    </>
  );
}
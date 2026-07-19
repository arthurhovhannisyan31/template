"use client";

import type { AuthResponse } from "generated/client";
import { useMemo } from "react";

import { authClient } from "lib/auth-client";

export const getAuthData = (): AuthResponse | undefined => {
  const { data: session, isPending } = authClient.useSession();

  if (!isPending && session) {
    if ("data" in session.user) {
      return session.user.data as AuthResponse;
    }
  }
};

export const useAuthData = (): AuthResponse | undefined => {
  const { data: session, isPending } = authClient.useSession();

  return useMemo(() => {
    if (!isPending && session) {
      if ("data" in session.user) {
        return session.user.data as AuthResponse;
      }
    }
  }, [isPending, session]);
};

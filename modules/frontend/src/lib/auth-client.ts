import { createAuthClient } from "better-auth/react";
import { API_BASE_URL } from "configs/constants";

export const authClient = createAuthClient({
  baseURL: API_BASE_URL,
});

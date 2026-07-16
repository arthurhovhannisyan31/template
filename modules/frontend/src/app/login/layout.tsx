import type { Metadata } from "next";
import type { PropsWithChildren } from "react";

import { cookies } from "next/headers";

export const metadata: Metadata = {
  title: "Login",
};

export default async function LoginLayout({ children }: PropsWithChildren) {
  const cookieStore = await cookies();
  const theme = cookieStore.get("theme");
  console.log({
    theme,
  });

  return <section>{children}</section>;
}

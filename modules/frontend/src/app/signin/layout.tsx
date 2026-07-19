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

  return (
    <section>
      <div className="flex min-h-svh w-full items-center justify-center p-6 md:p-10">
        <div className="w-full max-w-sm">{children}</div>
      </div>
    </section>
  );
}

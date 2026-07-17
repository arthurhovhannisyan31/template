import type { Metadata } from "next";
import type { PropsWithChildren } from "react";

export const metadata: Metadata = {
  title: "Sign up",
};

export default async function SignupLayout({ children }: PropsWithChildren) {
  return <section>{children}</section>;
}

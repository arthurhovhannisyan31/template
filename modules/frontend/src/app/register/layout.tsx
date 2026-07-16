import type { Metadata } from "next";
import type { PropsWithChildren } from "react";

export const metadata: Metadata = {
  title: "Registration",
};

export default async function RegisterLayout({ children }: PropsWithChildren) {
  return <section>{children}</section>;
}

import { type NextRequest, NextResponse } from "next/server";

export default async function proxy(
  _request: NextRequest,
): Promise<NextResponse | undefined> {
  return NextResponse.next();
}

export const config = {
  matcher: [
    {
      source:
        "/((?!api|_vercel|_next/static|_next/image|_next/data|favicon.ico|robots.txt).*)",
    },
  ],
  unstable_allowDynamic: ["**/node_modules/lodash-es/**/*.js"],
};

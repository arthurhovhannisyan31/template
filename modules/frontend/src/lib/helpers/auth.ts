import { isAfter } from "date-fns";
import { jwtVerify } from "jose";

export const getTokenExpired = async (token = ""): Promise<boolean> => {
  try {
    const secret = new TextEncoder().encode(process.env.BETTER_AUTH_SECRET);
    const { payload } = await jwtVerify(token, secret);
    const sessionExpirationDate = new Date((payload.exp as number) * 1000);

    return isAfter(Date.now(), sessionExpirationDate);
  } catch {
    return true;
  }
};

import zod from "zod";

export const loginSchema = zod.object({
  email: zod.string().email("Please enter a valid email address"),
  password: zod.string().min(8, "Password must be at least 8 characters long"),
});

export type LoginSchema = zod.output<typeof loginSchema>;

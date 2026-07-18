import zod from "zod";

export const signupSchema = zod
  .object({
    username: zod
      .string()
      .min(6, "Username must be at least 8 characters long"),
    email: zod.string().email("Please enter a valid email address"),
    password: zod
      .string()
      .min(8, "Password must be at least 8 characters long"),
    confirmPassword: zod
      .string()
      .min(8, "Password must be at least 8 characters long"),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords do not match",
    path: ["confirmPassword"],
  });

export type SignupSchema = zod.output<typeof signupSchema>;

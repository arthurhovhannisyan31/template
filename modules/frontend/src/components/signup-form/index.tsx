"use client";

import { useState } from "react";
import { type SubmitHandler, useForm } from "react-hook-form";

import { zodResolver } from "@hookform/resolvers/zod";
import {
  type SignupSchema,
  signupSchema,
} from "components/signup-form/constants";
import { Button } from "components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "components/ui/card";
import {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
} from "components/ui/field";
import { Input } from "components/ui/input";
import { PasswordInput } from "components/ui/password-input";
import { RootPath } from "configs/routes/constants";
import { authClient } from "lib/auth-client";
import { cn } from "lib/utils";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { toast } from "sonner";

export function SignupForm({
  className,
  ...props
}: React.ComponentProps<"div">) {
  const [isLoading, setIsLoading] = useState(false);
  const router = useRouter();

  const {
    register,
    handleSubmit,
    formState: { errors, dirtyFields },
  } = useForm<SignupSchema>({
    defaultValues: {
      username: "",
      email: "",
      password: "",
      confirmPassword: "",
    },
    mode: "onTouched",
    resolver: zodResolver(signupSchema),
  });

  const onSubmit: SubmitHandler<SignupSchema> = async ({
    email,
    password,
    username,
  }) => {
    await authClient.signUp.email(
      {
        email,
        password,
        name: username,
        callbackURL: "/",
      },
      {
        onRequest: () => {
          setIsLoading(true);
        },
        onSuccess: (ctx) => {
          // TODO Store token in some store to use in axios
          // Set user data to store, test if several calls to session cause several api calls
          router.push("/");
        },
        onError: (ctx) => {
          setIsLoading(false);
          toast.error(
            (ctx.error.message || ctx.response.statusText).toString(),
          );
        },
      },
    );
  };

  return (
    <div className={cn("flex flex-col gap-6", className)} {...props}>
      <Card {...props}>
        <CardHeader>
          <CardTitle>Create an account</CardTitle>
          <CardDescription>
            Enter your information below to create your account
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit(onSubmit)}>
            <FieldGroup>
              <Field>
                <FieldLabel htmlFor="name">Username</FieldLabel>
                <Input
                  id="name"
                  type="text"
                  placeholder="johndoe"
                  required
                  autoComplete="username"
                  {...register("username")}
                />
                <FieldError errors={[errors.username]} />
              </Field>
              <Field>
                <FieldLabel htmlFor="email">Email</FieldLabel>
                <Input
                  id="email"
                  type="email"
                  placeholder="m@example.com"
                  required
                  autoComplete="email"
                  {...register("email")}
                />
                <FieldError errors={[errors.email]} />
                <FieldDescription>
                  We&apos;ll use this to contact you. We will not share your
                  email with anyone else.
                </FieldDescription>
              </Field>
              <Field>
                <FieldLabel htmlFor="password">Password</FieldLabel>
                <PasswordInput
                  id="password"
                  required
                  isDirty={!!dirtyFields.password}
                  autoComplete="new-password"
                  {...register("password")}
                />
                <FieldError errors={[errors.password]} />
                <FieldDescription>
                  Must be at least 8 characters long.
                </FieldDescription>
              </Field>
              <Field>
                <FieldLabel htmlFor="confirm-password">
                  Confirm Password
                </FieldLabel>
                <PasswordInput
                  id="confirm-password"
                  required
                  isDirty={!!dirtyFields.confirmPassword}
                  autoComplete="new-password"
                  {...register("confirmPassword")}
                />
                <FieldError errors={[errors.confirmPassword]} />
                <FieldDescription>
                  Please confirm your password.
                </FieldDescription>
              </Field>
              <FieldGroup>
                <Field>
                  <Button disabled={isLoading} type="submit">
                    Create Account
                  </Button>
                  <FieldDescription className="px-6 text-center">
                    Already have an account?{" "}
                    <Link href={RootPath.SignIn}>Sign in</Link>
                  </FieldDescription>
                </Field>
              </FieldGroup>
            </FieldGroup>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}

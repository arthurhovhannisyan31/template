"use client";

import * as React from "react";

import { Button } from "app/components/ui/button";
import { Input } from "app/components/ui/input";
import { Eye, EyeOff } from "lucide-react";

export interface PasswordInputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {
  isDirty: boolean;
  ref?: React.Ref<HTMLInputElement>;
}

const PasswordInput: React.FC<PasswordInputProps> = ({
  className,
  isDirty,
  ref,
  ...props
}) => {
  const [showPassword, setShowPassword] = React.useState(false);

  return (
    <div className="relative">
      <Input
        type={showPassword ? "text" : "password"}
        className={`pr-10 ${className}`}
        ref={ref}
        {...props}
      />
      <Button
        type="button"
        variant="ghost"
        size="sm"
        className="absolute right-0 top-0 h-full px-3 py-2 hover:bg-transparent"
        onClick={() => setShowPassword((prev) => !prev)}
        disabled={!isDirty}
      >
        {showPassword ? (
          <EyeOff
            className="h-4 w-4 text-muted-foreground"
            aria-hidden="true"
          />
        ) : (
          <Eye className="h-4 w-4 text-muted-foreground" aria-hidden="true" />
        )}
        <span className="sr-only">
          {showPassword ? "Hide password" : "Show password"}
        </span>
      </Button>
    </div>
  );
};

export { PasswordInput };

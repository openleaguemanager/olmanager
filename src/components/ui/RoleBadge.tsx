import { getRoleIconPath, getRoleBadgeVariant, getRoleAbbreviation, LolRole } from "../../lib/roleIcons";

interface RoleBadgeProps {
  role: string;
  size?: "sm" | "md" | "lg";
  showLabel?: boolean;
  className?: string;
  title?: string;
}

/**
 * Reusable badge component that displays a role icon with optional label.
 * Automatically handles color coding and abbreviations.
 * 
 * @example
 * // Icon only (compact)
 * <RoleBadge role="JUNGLE" />
 * 
 * @example
 * // Icon with label
 * <RoleBadge role="ADC" showLabel />
 * 
 * @example
 * // Custom size
 * <RoleBadge role="MID" size="lg" />
 */
export function RoleBadge({ 
  role, 
  size = "sm", 
  showLabel = false,
  className = "",
  title
}: RoleBadgeProps) {
  const iconPath = getRoleIconPath(role);
  const variant = getRoleBadgeVariant(role);
  const abbreviation = getRoleAbbreviation(role);
  
  // Use provided title or default to the full role name
  const tooltip = title ?? role;

  const variants = {
    primary: "bg-primary-100 text-primary-700 dark:bg-primary-900/40 dark:text-primary-300",
    accent: "bg-accent-100 text-accent-700 dark:bg-accent-900/40 dark:text-accent-300",
    success: "bg-green-100 text-green-700 dark:bg-green-900/40 dark:text-green-400",
    danger: "bg-red-100 text-red-700 dark:bg-red-900/40 dark:text-red-400",
    neutral: "bg-gray-100 text-gray-600 dark:bg-navy-600 dark:text-gray-400",
  };

  const sizes = {
    sm: "px-2 py-0.5 text-xs",
    md: "px-3 py-1 text-sm",
    lg: "px-4 py-1.5 text-base",
  };

  // Fallback to text badge if role is not recognized
  if (!iconPath) {
    return (
      <span
        className={`inline-flex items-center font-bold font-heading uppercase tracking-wider rounded-md ${variants.neutral} ${sizes[size]} ${className}`}
        title={tooltip}
      >
        {role}
      </span>
    );
  }

  return (
    <span
      className={`inline-flex items-center gap-1 font-bold font-heading uppercase tracking-wider rounded-md ${variants[variant]} ${sizes[size]} ${className}`}
      title={tooltip}
    >
      <img 
        src={iconPath} 
        alt={role} 
        className={size === "sm" ? "h-3.5 w-3.5" : size === "md" ? "h-4 w-4" : "h-5 w-5"} 
      />
      {showLabel && (
        <span className="leading-none">
          {abbreviation}
        </span>
      )}
    </span>
  );
}

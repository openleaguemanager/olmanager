import { Badge } from "./Badge";
import { getRoleIconPath, getRoleBadgeVariant, getRoleAbbreviation, LolRole } from "../lib/roleIcons";

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

  if (!iconPath) {
    // Fallback to text badge if role is not recognized
    return (
      <Badge variant="neutral" size={size} className={className}>
        {role}
      </Badge>
    );
  }

  return (
    <Badge 
      variant={variant} 
      size={size} 
      className={`flex items-center gap-1 ${className}`}
      title={tooltip}
    >
      <img 
        src={iconPath} 
        alt={role} 
        className={size === "sm" ? "h-3.5 w-3.5" : size === "md" ? "h-4 w-4" : "h-5 w-5"} 
      />
      {showLabel && (
        <span className="font-heading font-bold uppercase tracking-wide">
          {abbreviation}
        </span>
      )}
    </Badge>
  );
}

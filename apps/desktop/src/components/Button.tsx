import type { ButtonHTMLAttributes, ReactNode } from 'react'
type Props = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: 'primary' | 'glass' | 'ghost' | 'danger'
  icon?: ReactNode
}
export function Button({ variant = 'primary', icon, children, className = '', ...rest }: Props) {
  return (
    <button type="button" className={`btn btn--${variant} ${className}`.trim()} {...rest}>
      {icon}
      {children}
    </button>
  )
}

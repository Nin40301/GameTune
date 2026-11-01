import type { InputHTMLAttributes } from 'react'
import { Icon } from './Icon'

type Props = Omit<InputHTMLAttributes<HTMLInputElement>, 'type'>
export function Checkbox({ className = '', ...props }: Props) {
  return (
    <span className="checkbox-control">
      <input {...props} type="checkbox" className={`checkbox ${className}`.trim()} />
      <Icon name="check" />
    </span>
  )
}

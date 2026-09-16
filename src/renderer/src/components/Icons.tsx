import type { SVGProps } from 'react'

type IconProps = SVGProps<SVGSVGElement>

const Icon = ({ children, ...props }: IconProps): React.JSX.Element => (
  <svg
    aria-hidden="true"
    fill="none"
    height="20"
    viewBox="0 0 24 24"
    width="20"
    {...props}
  >
    {children}
  </svg>
)

export const TableIcon = (props: IconProps): React.JSX.Element => (
  <Icon {...props}>
    <rect height="17" rx="2" stroke="currentColor" strokeWidth="1.8" width="19" x="2.5" y="3.5" />
    <path d="M2.5 9h19M8.5 3.5v17" stroke="currentColor" strokeWidth="1.8" />
  </Icon>
)

export const ClockIcon = (props: IconProps): React.JSX.Element => (
  <Icon {...props}>
    <circle cx="12" cy="12" r="9" stroke="currentColor" strokeWidth="1.8" />
    <path d="M12 7v5l3.5 2" stroke="currentColor" strokeLinecap="round" strokeWidth="1.8" />
  </Icon>
)

export const SettingsIcon = (props: IconProps): React.JSX.Element => (
  <Icon {...props}>
    <path
      d="M9.6 3.6 10.2 2h3.6l.6 1.6 1.6.7 1.6-.7 2.5 2.5-.7 1.6.7 1.6 1.6.6v3.6l-1.6.6-.7 1.6.7 1.6-2.5 2.5-1.6-.7-1.6.7-.6 1.6h-3.6l-.6-1.6-1.6-.7-1.6.7-2.5-2.5.7-1.6-.7-1.6-1.6-.6V9.9l1.6-.6.7-1.6-.7-1.6 2.5-2.5 1.6.7 1.6-.7Z"
      stroke="currentColor"
      strokeLinejoin="round"
      strokeWidth="1.5"
    />
    <circle cx="12" cy="11.8" r="3" stroke="currentColor" strokeWidth="1.6" />
  </Icon>
)

export const FileIcon = (props: IconProps): React.JSX.Element => (
  <Icon {...props}>
    <path d="M6 2.8h8l4 4v14.4H6V2.8Z" stroke="currentColor" strokeLinejoin="round" strokeWidth="1.6" />
    <path d="M14 2.8v4h4" stroke="currentColor" strokeLinejoin="round" strokeWidth="1.6" />
  </Icon>
)

export const AlertIcon = (props: IconProps): React.JSX.Element => (
  <Icon {...props}>
    <circle cx="12" cy="12" r="10" fill="currentColor" />
    <path d="M12 6.8v6.5M12 17.1v.1" stroke="white" strokeLinecap="round" strokeWidth="2" />
  </Icon>
)

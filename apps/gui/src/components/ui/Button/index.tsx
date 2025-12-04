import './styles.css'

export default function Button({
    icon,
    activeOption = false,
    disabled = false,
    func = undefined,
    ...rest
}: {
    icon: string,
    activeOption?: boolean,
    disabled?: boolean,
    func?: () => void,
}) {
  return (
    <button
        className={`button${activeOption ? ' active' : ''}`}
        disabled={disabled}
        onClick={func}
        {...rest}
    >
        <span className="material-symbols-rounded">{icon}</span>
    </button>
  )
}

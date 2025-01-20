import startCase from 'lodash.startcase'

export function toTitleCase(str: string): string {
	return startCase(str.toLowerCase())
}

export function toYesNo(value: boolean | undefined): string {
	return value ? 'Sí' : 'No'
}

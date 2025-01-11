import startCase from 'lodash.startcase'

export function toTitleCase(str: string): string {
	return startCase(str.toLowerCase())
}

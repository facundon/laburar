import { eachDayOfInterval, endOfYear, format, isSaturday, isSunday, parse, startOfYear } from 'date-fns'
import { es } from 'date-fns/locale'
import startCase from 'lodash.startcase'
import { SvelteDate } from 'svelte/reactivity'

export function toTitleCase(str: string): string {
	return startCase(str.toLowerCase())
}

export function toYesNo(value: boolean | undefined): string {
	return value ? 'Sí' : 'No'
}

export function getWeekends(includeSaturdays: boolean = true): Date[] {
	const date = new Date()
	const start = startOfYear(date)
	const end = endOfYear(date)
	const allDays = eachDayOfInterval({ start, end })

	return allDays.filter(date => isSunday(date) || (includeSaturdays && isSaturday(date)))
}

export function parseDate(date: string, isReactive = false): Date {
	return parse(date, 'yyyy-MM-dd', isReactive ? new SvelteDate() : new Date(), { locale: es })
}

export function formatDate(date: Date) {
	return format(date, 'yyyy-MM-dd', { locale: es })
}

export function formatDateTime(date: Date) {
	return format(date, 'yyyy-MM-dd HH:mm', { locale: es })
}

export function formatDateToFullDay(date: Date) {
	const formatedDate = format(date, 'eeee dd MMMM', { locale: es })
	return formatedDate
		.split(' ')
		.map((word, index) => {
			if (index === 1) return word
			return word.charAt(0).toUpperCase() + word.slice(1)
		})
		.join(' ')
}

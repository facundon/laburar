import { invoke } from '$invoke'
import { Holiday, type HolidayDTO } from '$models/holiday.svelte'

export async function getHolidays() {
	try {
		return invoke('list_holidays_command', undefined, (data: HolidayDTO[]) => data.map(Holiday.fromDTO))
	} catch (err) {
		console.error('Failed to fetch holidays:', err)
		return []
	}
}

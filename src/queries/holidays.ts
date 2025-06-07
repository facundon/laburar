import { invoke } from '$invoke'
import { Holiday, type HolidayDTO } from '$models/holiday.svelte'

export async function getHolidays(year?: number) {
	try {
		return invoke('list_holidays_command', { year }, (data: HolidayDTO[]) => data.map(Holiday.fromDTO))
	} catch (err) {
		console.error('Failed to fetch holidays:', err)
		return []
	}
}

export async function listHolidaysForEmployee(employeeId: number) {
	try {
		return invoke('list_holidays_for_employee_command', { employee_id: employeeId }, (data: HolidayDTO[]) => data.map(Holiday.fromDTO))
	} catch (err) {
		console.error('Failed to fetch holidays for employee:', err)
		return []
	}
}

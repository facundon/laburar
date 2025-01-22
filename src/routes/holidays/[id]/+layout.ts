import { breadcrumbData } from '$context'
import { invoke } from '$invoke'
import { Holiday } from '$models/holiday.svelte.js'

export const load = async ({ params }) => {
	const holidayId = Number(params.id)
	try {
		const holiday = await invoke('get_holiday_command', { id: holidayId }, Holiday.fromDTO)
		breadcrumbData.name = `${holiday.startDate.toLocaleDateString()} - ${holiday.endDate.toLocaleDateString()}`
		return { holiday }
	} catch (err) {
		console.error('Failed to fetch holiday:', err)
		return { holiday: null }
	}
}

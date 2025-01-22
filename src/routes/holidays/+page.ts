import { getHolidays } from '$queries/holidays'

export const load = async () => {
	const holidays = await getHolidays()
	return { holidays }
}

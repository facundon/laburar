import { getHolidays } from '$queries/holidays'

export const load = async ({ url }) => {
	const year = url.searchParams.get('year') || new Date().getFullYear()
	const holidays = await getHolidays(Number(year))
	return { holidays }
}

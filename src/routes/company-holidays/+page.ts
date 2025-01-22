import { getCompanyHolidays } from '$queries/companyHolidays'

export const load = async () => {
	const companyHolidays = await getCompanyHolidays()
	return { companyHolidays }
}

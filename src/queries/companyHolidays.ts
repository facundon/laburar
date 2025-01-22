import { invoke } from '$invoke'
import { CompanyHoliday, type CompanyHolidayDTO } from '$models/company_holiday.svelte'

export async function getCompanyHolidays() {
	try {
		return invoke('list_company_holidays_command', undefined, (data: CompanyHolidayDTO[]) => data.map(CompanyHoliday.fromDTO))
	} catch (err) {
		console.error('Failed to fetch company holidays:', err)
		return []
	}
}

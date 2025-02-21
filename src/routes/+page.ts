import { listEmployeeFutureAbsences, listEmployeesOnHolidays } from '$queries/employees'
import { listReplacements } from '$queries/replacements'

export const load = async () => {
	const employeesOnHoliday = await listEmployeesOnHolidays()
	const employeesFutureAbsences = await listEmployeeFutureAbsences()
	const replacements = await listReplacements(null, false)

	return { employeesOnHoliday, employeesFutureAbsences, replacements }
}

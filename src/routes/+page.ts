import { invoke } from '$invoke'
import { listEmployeeFutureAbsences, listEmployeesOnHolidays } from '$queries/employees'

export const load = async () => {
	try {
		await invoke('delete_finished_replacements_command')
	} catch (err) {
		console.error(`Error deleting finished replacements: `, err)
	}
	const employeesOnHoliday = await listEmployeesOnHolidays()
	const employeesFutureAbsences = await listEmployeeFutureAbsences()
	return { employeesOnHoliday, employeesFutureAbsences }
}

import { listEmployeeFutureAbsences, listEmployeesOnHolidays } from '$queries/employees'

export const load = async () => {
	const employeesOnHoliday = await listEmployeesOnHolidays()
	const employeesFutureAbsences = await listEmployeeFutureAbsences()
	return { employeesOnHoliday, employeesFutureAbsences }
}

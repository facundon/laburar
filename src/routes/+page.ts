import { listAssignmentsWithoutEmployees } from '$queries/assignments'
import { listEmployeesOnHolidays } from '$queries/employees'

export const load = async () => {
	const employeesOnHoliday = await listEmployeesOnHolidays()
	return { employeesOnHoliday }
}

import { getEmployeeList } from '$queries/employees'

export const load = async () => {
	const employees = await getEmployeeList()
	return { employees }
}

import { getEmployeeList } from '$queries/employees/getEmployeeList'

export const load = async () => {
	const employees = await getEmployeeList()
	return { employees }
}

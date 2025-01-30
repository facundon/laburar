import { breadcrumbData } from '$context'
import { getEmployeeWithAssignments } from '$queries/employees'

export const prerender = false

export const load = async ({ params }) => {
	const employee = await getEmployeeWithAssignments(Number(params.id))
	if (employee) breadcrumbData.name = employee.name
	return { employee }
}

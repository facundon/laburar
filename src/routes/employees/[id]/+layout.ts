import { breadcrumbData } from '$context'
import { getEmployee } from '$queries/employees/getEmployee'

export const load = async ({ params }) => {
	const employee = await getEmployee(Number(params.id))
	if (employee) breadcrumbData.name = employee.name
	return { employee }
}

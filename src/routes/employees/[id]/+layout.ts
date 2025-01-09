import { getEmployee } from '$queries/employees/getEmployee'

export const load = async ({ params }) => {
	const employee = await getEmployee(Number(params.id))
	return { employee }
}

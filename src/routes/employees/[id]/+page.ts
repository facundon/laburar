import { getAssignmentsForEmployee } from '$queries/assignments/getAssignmentsForEmployee'

export const load = async ({ params }) => {
	const assignments = await getAssignmentsForEmployee(Number(params.id))
	return { assignments }
}

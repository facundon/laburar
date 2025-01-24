import { listAssignmentsWithoutEmployees } from '$queries/assignments'

export const load = async () => {
	const assignments = await listAssignmentsWithoutEmployees()
	return { assignments }
}

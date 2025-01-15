import { getAssignmentList } from '$queries/assignments'

export const load = async ({ parent }) => {
	const { employee } = await parent()
	let assignments = await getAssignmentList()
	assignments = assignments.filter(assignment => !employee?.assignments?.some(({ assignmentId }) => assignment.id === assignmentId))
	return { assignments }
}

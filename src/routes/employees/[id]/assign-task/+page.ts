import { getAssignmentList } from '$queries/assignments/getAssignmentList'

export const load = async () => {
	const assignments = await getAssignmentList()
	return { assignments }
}

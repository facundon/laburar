import { getAssignmentList } from '$queries/assignments'

export const load = async () => {
	const assignments = await getAssignmentList()
	return { assignments }
}

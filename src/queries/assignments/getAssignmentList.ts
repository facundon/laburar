import { invoke } from '$invoke'
import { Assignment, type AssignmentDTO } from '$models/assignment'

export async function getAssignmentList() {
	try {
		return invoke('list_assignments_command', undefined, (data: AssignmentDTO[]) => data.map(Assignment.fromDTO))
	} catch (error) {
		console.error('Failed to fetch areas:', error)
		return []
	}
}

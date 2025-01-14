import { invoke } from '$invoke'
import { type AssignmentDTO, Assignment } from '$models/assignment'

export async function getAssignmentList() {
	try {
		return invoke('list_assignments_command', undefined, (data: AssignmentDTO[]) => data.map(Assignment.fromDTO))
	} catch (error) {
		console.error('Failed to fetch areas:', error)
		return []
	}
}
export async function getAssignmentsForEmployee(employeeId: number) {
	try {
		const assignments = await invoke('list_employee_assignments_command', { id: employeeId }, (data: AssignmentDTO[]) =>
			data.map(Assignment.fromDTO),
		)
		return assignments
	} catch (err) {
		console.error(err)
		return []
	}
}

import { invoke } from '$invoke'
import { Assignment, type AssignmentDTO } from '$models/assignment'

export async function getAssignmentsForEmployee(employeeId: number) {
	try {
		const assignments = await invoke('list_employee_assignments_command', { employee_id: employeeId }, (data: AssignmentDTO[]) =>
			data.map(Assignment.fromDTO),
		)
		return assignments
	} catch (err) {
		console.error(err)
		return []
	}
}

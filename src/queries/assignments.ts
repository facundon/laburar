import { invoke } from '$invoke'
import { type AssignmentDTO, Assignment } from '$models/assignment.svelte'
import { SuggestedEmployee, type SuggestedEmployeeDTO } from '$models/employee.svelte'

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

export async function listAssignmentsWithoutEmployees() {
	try {
		return invoke('list_assignments_without_employees_command', undefined, (data: AssignmentDTO[]) => data.map(Assignment.fromDTO))
	} catch (error) {
		console.error('Failed to fetch assignments:', error)
		return []
	}
}

export async function suggestEmployeesForAssignment(assignmentId: number, startDate: string, endDate: string, currentEmployeeId: number) {
	try {
		return invoke(
			'suggest_employees_for_assignation_command',
			{
				assignment_id: assignmentId,
				assignation_start_date: startDate,
				assignation_end_date: endDate,
				current_employee_id: currentEmployeeId,
			},
			(data: SuggestedEmployeeDTO[]) => data.slice(0, 3).map(SuggestedEmployee.fromDTO),
		)
	} catch (err) {
		console.error('Failed to get suggestions for assignation', err)
		return []
	}
}

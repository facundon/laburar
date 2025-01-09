import { invoke } from '$invoke'
import { Employee } from '$models/employee'

export async function getEmployee(id: number) {
	try {
		return invoke('get_employee_command', { id }, Employee.fromDTO)
	} catch (error) {
		console.error('Failed to fetch employee:', error)
		return null
	}
}

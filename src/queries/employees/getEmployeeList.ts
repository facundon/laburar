import { invoke } from '$invoke'
import { Employee, type EmployeeDTO } from '$models/employee'

export async function getEmployeeList() {
	try {
		return invoke('list_employees_command', undefined, (data: EmployeeDTO[]) => data.map(Employee.fromDTO))
	} catch (error) {
		console.error('Failed to fetch employees:', error)
		return []
	}
}

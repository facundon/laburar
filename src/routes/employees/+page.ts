import { invoke } from '$invoke'
import { Employee, type EmployeeDTO } from '$models/employee'

export const load = async () => {
	try {
		const employees = await invoke('list_employees_command', undefined, (data: EmployeeDTO[]) => data.map(Employee.fromDTO))
		return { employees }
	} catch (error) {
		console.error('Failed to fetch employees:', error)
		return { employees: [] }
	}
}

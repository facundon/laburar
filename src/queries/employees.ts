import { invoke } from '$invoke'
import { Employee, EmployeeOnHoliday, type EmployeeDTO, type EmployeeOnHolidayDTO } from '$models/employee.svelte'

export async function getEmployee(id: number) {
	try {
		return invoke('get_employee_command', { id }, Employee.fromDTO)
	} catch (error) {
		console.error('Failed to fetch employee:', error)
		return null
	}
}
export async function getEmployeeList() {
	try {
		return invoke('list_employees_command', undefined, (data: EmployeeDTO[]) => data.map(Employee.fromDTO))
	} catch (error) {
		console.error('Failed to fetch employees:', error)
		return []
	}
}

export async function getEmployeeWithAssignments(id: number) {
	try {
		return invoke('get_employee_with_assignments_command', { id }, Employee.fromDTO)
	} catch (error) {
		console.error('Failed to fetch employee with assignments:', error)
		return null
	}
}

export async function listEmployeesOnHolidays() {
	try {
		return invoke('list_employees_on_holidays_command', undefined, (data: EmployeeOnHolidayDTO[]) => data.map(EmployeeOnHoliday.fromDTO))
	} catch (error) {
		console.error('Failed to fetch employees on holiday:', error)
		return []
	}
}

export async function listEmployeeFutureAbsences() {
	try {
		return invoke('list_employees_future_absences_command', undefined, (data: EmployeeOnHolidayDTO[]) =>
			data.map(EmployeeOnHoliday.fromDTO),
		)
	} catch (error) {
		console.error('Failed to fetch employees on holiday:', error)
		return []
	}
}

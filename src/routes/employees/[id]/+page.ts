import { invoke } from '$invoke'
import { Employee } from '$models/employee'

export const load = async ({ params }) => {
	try {
		const employee = await invoke('get_employee_command', { id: Number(params.id) }, Employee.fromDTO)
		return { employee }
	} catch (error) {
		console.error('Failed to fetch employee:', error)
		return { employee: null }
	}
}

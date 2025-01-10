import { invoke } from '$invoke'
import { Task, type TaskDTO } from '$models/task'

export async function getTasksForEmployee(employeeId: number) {
	try {
		const tasks = await invoke('get_tasks_for_employee_command', { employee_id: employeeId }, (data: TaskDTO[]) => data.map(Task.fromDTO))
		return tasks
	} catch (err) {
		console.error(err)
		return []
	}
}

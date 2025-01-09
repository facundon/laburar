import { invoke } from '$invoke'
import { type TaskDTO, Task } from '$models/task'

export async function getTaskList() {
	try {
		return invoke('list_tasks_command', undefined, (data: TaskDTO[]) => data.map(Task.fromDTO))
	} catch (error) {
		console.error('Failed to fetch tasks:', error)
		return []
	}
}

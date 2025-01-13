import { invoke } from '$invoke'
import { type TaskDTO, Task } from '$models/task'

export async function getTaskList(excludeIds: number[] = []) {
	try {
		return invoke('list_tasks_command', { exclude_ids: excludeIds }, (data: TaskDTO[]) => data.map(Task.fromDTO))
	} catch (error) {
		console.error('Failed to fetch tasks:', error)
		return []
	}
}

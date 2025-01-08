import { invoke } from '$invoke'
import { Task, type TaskDTO } from '$models/task'

export const load = async () => {
	try {
		const tasks = await invoke('list_tasks_command', undefined, (data: TaskDTO[]) => data.map(Task.fromDTO))
		return { tasks }
	} catch (error) {
		console.error('Failed to fetch tasks:', error)
		return { tasks: [] }
	}
}

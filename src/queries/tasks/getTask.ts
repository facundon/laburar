import { invoke } from '$invoke'
import { Task } from '$models/task'

export async function getTask(id: number) {
	try {
		return invoke('get_task_command', { id }, Task.fromDTO)
	} catch (error) {
		console.error('Failed to fetch task:', error)
		return null
	}
}

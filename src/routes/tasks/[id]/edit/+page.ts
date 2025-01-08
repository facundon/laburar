import { invoke } from '$invoke'
import { Task } from '$models/task.js'

export const load = async ({ params }) => {
	try {
		const task = await invoke('get_task_command', { id: Number(params.id) }, Task.fromDTO)
		return { task }
	} catch (error) {
		console.error('Failed to fetch task:', error)
		return { task: null }
	}
}

import { invoke } from '$invoke'
import { Task, type TaskDTO } from '$models/task.svelte'

export async function getTask(id: number) {
	try {
		return invoke('get_task_command', { id }, Task.fromDTO)
	} catch (error) {
		console.error('Failed to fetch task:', error)
		return null
	}
}
export async function getTaskList(excludeIds: number[] = []) {
	try {
		return invoke('list_tasks_command', { exclude_ids: excludeIds }, (data: TaskDTO[]) => data.map(Task.fromDTO))
	} catch (error) {
		console.error('Failed to fetch tasks:', error)
		return []
	}
}
export async function getTasksForArea(areaId: number) {
	try {
		return invoke('get_tasks_for_area_command', { area_id: areaId }, (data: TaskDTO[]) => data.map(Task.fromDTO))
	} catch (error) {
		console.error('Failed to fetch tasks for area:', error)
		return []
	}
}

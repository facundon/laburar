import { invoke } from '$invoke'
import { Task, type TaskDTO } from '$models/task'

export async function getTasksForArea(areaId: number) {
	try {
		return invoke('get_tasks_for_area_command', { area_id: areaId }, (data: TaskDTO[]) => data.map(Task.fromDTO))
	} catch (error) {
		console.error('Failed to fetch tasks for area:', error)
		return []
	}
}

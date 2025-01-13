import { invoke } from '$invoke'
import { Area } from '$models/area'

export async function getAreaWithTasks(id: number) {
	try {
		return invoke('get_area_with_tasks_command', { id }, Area.fromDTO)
	} catch (error) {
		console.error('Failed to fetch area with tasks:', error)
		return null
	}
}

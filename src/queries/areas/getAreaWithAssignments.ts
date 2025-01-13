import { invoke } from '$invoke'
import { Area, type AreaDTO } from '$models/area'

export async function getAreaWithAssignments(id: number) {
	try {
		return invoke('get_area_with_assignments_command', { id }, Area.fromDTO)
	} catch (error) {
		console.error('Failed to fetch area with tasks:', error)
		return null
	}
}

import { invoke } from '$invoke'
import { Area, type AreaDTO } from '$models/area.svelte'

export async function getArea(id: number) {
	try {
		return invoke('get_area_command', { id }, Area.fromDTO)
	} catch (error) {
		console.error('Failed to fetch area:', error)
		return null
	}
}

export async function getAreaList() {
	try {
		return invoke('list_areas_command', undefined, (data: AreaDTO[]) => data.map(Area.fromDTO))
	} catch (error) {
		console.error('Failed to fetch areas:', error)
		return []
	}
}

export async function getAreaWithAssignments(id: number) {
	try {
		return invoke('get_area_with_assignments_command', { id }, Area.fromDTO)
	} catch (error) {
		console.error('Failed to fetch area with tasks:', error)
		return null
	}
}

export async function getAreaListWithoutTasks() {
	try {
		return invoke('list_areas_without_tasks_command', undefined, (data: AreaDTO[]) => data.map(Area.fromDTO))
	} catch (error) {
		console.error('Failed to fetch areas without tasks:', error)
		return []
	}
}

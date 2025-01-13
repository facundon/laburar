import { invoke } from '$invoke'
import { Area } from '$models/area'

export async function getArea(id: number) {
	try {
		return invoke('get_area_command', { id }, Area.fromDTO)
	} catch (error) {
		console.error('Failed to fetch area:', error)
		return null
	}
}

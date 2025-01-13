import { invoke } from '$invoke'
import { type AreaDTO, Area } from '$models/area'

export async function getAreaList() {
	try {
		return invoke('list_areas_command', undefined, (data: AreaDTO[]) => data.map(Area.fromDTO))
	} catch (error) {
		console.error('Failed to fetch areas:', error)
		return []
	}
}

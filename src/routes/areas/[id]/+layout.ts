import { breadcrumbData } from '$context'
import { getAreaWithTasks } from '$queries/areas/getAreaWithTasks'

export const load = async ({ params }) => {
	const area = await getAreaWithTasks(Number(params.id))
	if (area) breadcrumbData.name = area.name
	return { area }
}

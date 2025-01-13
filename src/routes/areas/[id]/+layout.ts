import { breadcrumbData } from '$context'
import { getAreaWithAssignments } from '$queries/areas/getAreaWithAssignments'

export const load = async ({ params }) => {
	const area = await getAreaWithAssignments(Number(params.id))
	if (area) breadcrumbData.name = area.name
	return { area }
}

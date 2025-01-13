import { breadcrumbData } from '$context'
import { getArea } from '$queries/areas/getArea'

export const load = async ({ params }) => {
	const area = await getArea(Number(params.id))
	if (area) breadcrumbData.name = area.name
	return { area }
}

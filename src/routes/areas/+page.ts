import { getAreaList } from '$queries/areas/getAreaList'

export const load = async () => {
	const areas = await getAreaList()
	return { areas }
}

import { getAreaList } from '$queries/areas'

export const load = async () => {
	const areas = await getAreaList()
	return { areas }
}

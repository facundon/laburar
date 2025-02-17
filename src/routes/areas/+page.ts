import { getAreaList, getAreaListWithoutTasks } from '$queries/areas'

export const load = async () => {
	const [areas, areasWithoutTasks] = await Promise.all([getAreaList(), getAreaListWithoutTasks()])
	return { areas, areasWithoutTasks }
}

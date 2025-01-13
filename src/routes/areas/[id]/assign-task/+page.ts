import { getTaskList } from '$queries/tasks/getTaskList'

export const load = async ({ parent }) => {
	const { area } = await parent()
	let tasks = await getTaskList()
	tasks = tasks.filter(task => !area?.tasks.some(areaTask => areaTask.id === task.id))
	return { tasks }
}

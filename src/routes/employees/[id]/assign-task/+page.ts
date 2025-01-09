import { getTaskList } from '$queries/tasks/getTaskList'

export const load = async () => {
	const tasks = await getTaskList()
	return { tasks }
}

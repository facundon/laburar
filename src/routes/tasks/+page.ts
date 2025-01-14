import { getTaskList } from '$queries/tasks'

export const load = async () => {
	const tasks = await getTaskList()
	return { tasks }
}

import { getTaskList, listTasksWithoutArea } from '$queries/tasks'

export const load = async () => {
	const [tasks, tasksWithoutArea] = await Promise.all([getTaskList(), listTasksWithoutArea()])
	return { tasks, tasksWithoutArea }
}

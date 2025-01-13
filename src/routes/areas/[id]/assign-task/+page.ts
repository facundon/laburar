import { getTaskList } from '$queries/tasks/getTaskList'

export const load = async ({ parent }) => {
	const { area } = await parent()
	let tasks = await getTaskList()
	tasks = tasks.filter(task => !area?.assignments.some(assignment => assignment.taskId === task.id))
	return { tasks }
}

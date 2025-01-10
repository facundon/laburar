import { getTasksForEmployee } from '$queries/tasks/getTasksForEmployee'

export const load = async ({ params }) => {
	const tasks = await getTasksForEmployee(Number(params.id))
	return { tasks }
}

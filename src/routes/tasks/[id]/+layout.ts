import { getTask } from '$queries/tasks/getTask'

export const load = async ({ params }) => {
	const task = await getTask(Number(params.id))
	return { task }
}

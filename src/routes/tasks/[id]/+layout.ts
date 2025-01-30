import { breadcrumbData } from '$context'
import { getTask } from '$queries/tasks'

export const prerender = false

export const load = async ({ params }) => {
	const task = await getTask(Number(params.id))
	if (task) breadcrumbData.name = task.name
	return { task }
}

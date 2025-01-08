import { invoke as tauriInvoke, type InvokeArgs } from '@tauri-apps/api/core'

type Command =
	| 'create_employee_command'
	| 'get_employee_command'
	| 'list_employees_command'
	| 'delete_employee_command'
	| 'update_employee_command'
	| 'create_task_command'
	| 'get_task_command'
	| 'list_tasks_command'
	| 'delete_task_command'
	| 'update_task_command'

export const invoke = async <T>(command: Command, params?: InvokeArgs, parser?: (data: any) => T): Promise<T> => {
	const response = await tauriInvoke<T>(command, params)
	return parser ? parser(response) : (response as T)
}

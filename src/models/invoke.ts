import { invoke as tauriInvoke, type InvokeArgs } from "@tauri-apps/api/core"

type Command = "create_employee_command"

export const invoke = async <T>(
  command: Command,
  params: InvokeArgs,
  parser?: (data: any) => T
): Promise<T> => {
  const response = await tauriInvoke<T>(command, params)
  return parser ? parser(response) : (response as T)
}

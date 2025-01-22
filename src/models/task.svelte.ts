import { parseDate } from '$utils'

type UpdateTaskDTO = Omit<TaskDTO, 'created_at'>
type CreateTaskDTO = Omit<UpdateTaskDTO, 'id'>

export type TaskDTO = {
	id: number
	name: string
	description: string
	created_at: string
}

export class Task {
	id: number = 0
	name: string = $state('')
	description: string = $state('')
	createdAt: Date = new Date()

	constructor(params?: Partial<Omit<Task, 'toCreateDTO' | 'toUpdateDTO'>>) {
		if (params?.id !== undefined) this.id = params.id
		if (params?.name !== undefined) this.name = params.name
		if (params?.description !== undefined) this.description = params.description
		if (params?.createdAt !== undefined) this.createdAt = params.createdAt
	}

	static fromDTO(dto: TaskDTO): Task {
		return new Task({
			id: dto.id,
			name: dto.name,
			description: dto.description,
			createdAt: parseDate(dto.created_at),
		})
	}

	public toCreateDTO(): CreateTaskDTO {
		return {
			name: this.name,
			description: this.description,
		}
	}

	public toUpdateDTO(): UpdateTaskDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}

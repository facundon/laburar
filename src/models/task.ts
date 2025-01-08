type CreateTaskDTO = {
	name: string
	description: string
}

type UpdateTaskDTO = CreateTaskDTO & {
	id: number
}

export type TaskDTO = {
	id: number
	name: string
	description: string
	created_at: string
}

export class Task {
	id: number
	name: string
	description: string
	createdAt: Date

	constructor(params: Omit<Task, 'toCreateDTO' | 'toUpdateDTO'>) {
		this.id = params.id
		this.name = params.name
		this.description = params.description
		this.createdAt = params.createdAt
	}

	static fromDTO(dto: TaskDTO): Task {
		return new Task({
			id: dto.id,
			name: dto.name,
			description: dto.description,
			createdAt: new Date(dto.created_at),
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
			id: this.id,
			name: this.name,
			description: this.description,
		}
	}
}

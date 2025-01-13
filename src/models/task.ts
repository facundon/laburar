type CreateTaskDTO = Omit<TaskDTO, 'id' | 'created_at'>
type UpdateTaskDTO = Omit<TaskDTO, 'created_at'>

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

	constructor(params?: Partial<Omit<Task, 'toCreateDTO' | 'toUpdateDTO'>>) {
		this.id = params?.id || 0
		this.name = params?.name || ''
		this.description = params?.description || ''
		this.createdAt = params?.createdAt || new Date()
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
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}

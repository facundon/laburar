type CreateTaskDTO = Omit<TaskDTO, 'id' | 'created_at'>
type UpdateTaskDTO = Omit<TaskDTO, 'created_at'>

export type TaskDTO = {
	id: number
	name: string
	description: string
	area: string
	difficulty: string
	frequency: string
	created_at: string
}

export class Task {
	id: number
	name: string
	description: string
	area: string
	difficulty: string
	frequency: string
	createdAt: Date

	constructor(params: Omit<Task, 'toCreateDTO' | 'toUpdateDTO'>) {
		this.id = params.id
		this.name = params.name
		this.description = params.description
		this.area = params.area
		this.difficulty = params.difficulty
		this.frequency = params.frequency
		this.createdAt = params.createdAt
	}

	static fromDTO(dto: TaskDTO): Task {
		return new Task({
			id: dto.id,
			name: dto.name,
			description: dto.description,
			area: dto.area,
			difficulty: dto.difficulty,
			frequency: dto.frequency,
			createdAt: new Date(dto.created_at),
		})
	}

	public toCreateDTO(): CreateTaskDTO {
		return {
			name: this.name,
			description: this.description,
			area: this.area,
			difficulty: this.difficulty,
			frequency: this.frequency,
		}
	}

	public toUpdateDTO(): UpdateTaskDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}

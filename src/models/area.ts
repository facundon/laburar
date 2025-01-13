import { Task, type TaskDTO } from '$models/task'

type CreateAreaDTO = Omit<AreaDTO, 'id' | 'created_at' | 'tasks'>
type UpdateAreaDTO = Omit<AreaDTO, 'created_at' | 'tasks'>

export type AreaDTO = {
	id: number
	name: string
	description?: string
	tasks?: TaskDTO[]
	created_at: string
}

export class Area {
	id: number
	name: string
	description?: string
	tasks: Task[]
	createdAt: Date

	constructor(params?: Partial<Omit<Area, 'toCreateDTO' | 'toUpdateDTO'>>) {
		this.id = params?.id || 0
		this.name = params?.name || ''
		this.description = params?.description || ''
		this.tasks = params?.tasks || []
		this.createdAt = params?.createdAt || new Date()
	}

	static fromDTO(dto: AreaDTO): Area {
		return new Area({
			id: dto.id,
			name: dto.name,
			description: dto.description,
			tasks: dto.tasks?.map(Task.fromDTO),
			createdAt: new Date(dto.created_at),
		})
	}

	public toCreateDTO(): CreateAreaDTO {
		return {
			name: this.name,
			description: this.description,
		}
	}

	public toUpdateDTO(): UpdateAreaDTO {
		return {
			...this.toCreateDTO(),
			id: this.id,
		}
	}
}

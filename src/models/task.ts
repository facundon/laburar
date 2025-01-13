import { toTitleCase } from '../utils'

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

export const TaskFrequency = {
	DIARIA: 'diaria',
	SEMANAL: 'semanal',
	MENSUAL: 'mensual',
} as const
export type TaskFrequency = ValueOf<typeof TaskFrequency>

export const TaskDifficulty = {
	TRIVIAL: 0,
	FACIL: 1,
	MEDIA: 2,
	COMPLICADO: 3,
	DIFICIL: 4,
} as const
export type TaskDifficulty = ValueOf<typeof TaskDifficulty>

export const TaskEfficiency = {
	MEDIOCRE: 0,
	BAJA: 1,
	MEDIA: 2,
	ALTA: 3,
	EXCELENTE: 4,
} as const
export type TaskEfficiency = ValueOf<typeof TaskEfficiency>

export const TaskDifficulties = Object.entries(TaskDifficulty).map(([label, value]) => ({ label: toTitleCase(label), value }))
export const TaskEfficiencies = Object.entries(TaskEfficiency).map(([label, value]) => ({ label: toTitleCase(label), value }))

import { toTitleCase } from '../utils'

type CreateTaskDTO = Omit<TaskDTO, 'id' | 'created_at'>
type UpdateTaskDTO = Omit<TaskDTO, 'created_at'>

export type TaskDTO = {
	id: number
	name: string
	description: string
	area: string
	difficulty: number
	frequency: string
	created_at: string
}

export class Task {
	id: number
	name: string
	description: string
	area: string
	difficulty: number
	frequency: string
	createdAt: Date

	constructor(params?: Partial<Omit<Task, 'toCreateDTO' | 'toUpdateDTO'>>) {
		this.id = params?.id || 0
		this.name = params?.name || ''
		this.description = params?.description || ''
		this.area = params?.area || ''
		this.difficulty = params?.difficulty || 1
		this.frequency = params?.frequency || ''
		this.createdAt = params?.createdAt || new Date()
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

export const TaskArea = {
	HEALTH: 'health',
	FINANCE: 'finance',
	EDUCATION: 'education',
	PERSONAL: 'personal',
} as const
export type TaskArea = ValueOf<typeof TaskArea>

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

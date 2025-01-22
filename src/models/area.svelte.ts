import { Assignment, type AssignmentDTO } from '$models/assignment.svelte'
import { parseDate } from '$utils'

type UpdateAreaDTO = Omit<AreaDTO, 'created_at' | 'assignments'>
type CreateAreaDTO = Omit<UpdateAreaDTO, 'id'>

export type AreaDTO = {
	id: number
	name: string
	description?: string
	assignments?: AssignmentDTO[]
	created_at: string
}

export class Area {
	id: number = 0
	name: string = $state('')
	description?: string = $state('')
	assignments: Assignment[] = $state([])
	createdAt: Date = new Date()

	constructor(params?: Partial<Omit<Area, 'toCreateDTO' | 'toUpdateDTO'>>) {
		if (params?.id !== undefined) this.id = params.id
		if (params?.name !== undefined) this.name = params.name
		if (params?.description !== undefined) this.description = params.description
		if (params?.assignments !== undefined) this.assignments = params.assignments
		if (params?.createdAt !== undefined) this.createdAt = params.createdAt
	}

	static fromDTO(dto: AreaDTO): Area {
		return new Area({
			id: dto.id,
			name: dto.name,
			description: dto.description,
			assignments: dto.assignments?.map(Assignment.fromDTO),
			createdAt: parseDate(dto.created_at),
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

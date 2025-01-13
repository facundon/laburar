type CreateAreaDTO = Omit<AreaDTO, 'id' | 'created_at'>
type UpdateAreaDTO = Omit<AreaDTO, 'created_at'>

export type AreaDTO = {
	id: number
	name: string
	description?: string
	created_at: string
}

export class Area {
	id: number
	name: string
	description?: string
	createdAt: Date

	constructor(params?: Partial<Omit<Area, 'toCreateDTO' | 'toUpdateDTO'>>) {
		this.id = params?.id || 0
		this.name = params?.name || ''
		this.description = params?.description || ''
		this.createdAt = params?.createdAt || new Date()
	}

	static fromDTO(dto: AreaDTO): Area {
		return new Area({
			id: dto.id,
			name: dto.name,
			description: dto.description,
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

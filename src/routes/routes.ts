export const ROUTES = {
	employee: {
		list: '/employees',
		create: '/employees/create',
		edit: (id: number) => `/employees/${id}/edit`,
		view: (id: number) => `/employees/${id}`,
	},
} as const

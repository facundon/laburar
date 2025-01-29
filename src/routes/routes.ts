export const ROUTES = {
	employee: {
		list: '/employees',
		create: '/employees/create',
		edit: (id: number) => `/employees/${id}/edit`,
		view: (id: number) => `/employees/${id}`,
		assignTasks: (id: number) => `/employees/${id}/assign-task`,
	},
	task: {
		list: '/tasks',
		create: '/tasks/create',
		edit: (id: number) => `/tasks/${id}/edit`,
		view: (id: number) => `/tasks/${id}`,
	},
	area: {
		list: '/areas',
		create: '/areas/create',
		edit: (id: number) => `/areas/${id}/edit`,
		view: (id: number) => `/areas/${id}`,
		assignTasks: (id: number) => `/areas/${id}/assign-task`,
	},
	absence: {
		list: '/absences',
		create: '/absences/create',
		edit: (id: number) => `/absences/${id}/edit`,
		view: (id: number) => `/absences/${id}`,
		return: (id: number) => `/absences/${id}/return`,
	},
	holiday: {
		list: '/holidays',
		create: '/holidays/create',
		edit: (id: number) => `/holidays/${id}/edit`,
		view: (id: number) => `/holidays/${id}`,
	},
	companyHoliday: {
		list: '/company-holidays',
		create: '/company-holidays/create',
	},
	replacement: {
		list: '/replacements',
	},
} as const

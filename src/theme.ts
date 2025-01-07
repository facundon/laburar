export const theme = {
	colors: {
		primary: {
			main: '#FFC107', // Mustard
			light: '#FFD54F',
			dark: '#FFA000',
			contrast: '#000000',
		},
		secondary: {
			main: '#4CAF50', // Green
			light: '#81C784',
			dark: '#388E3C',
			contrast: '#FFFFFF',
		},
		tertiary: {
			main: '#0288D1', // Light Blue
			light: '#03A9F4',
			dark: '#0056b3',
			contrast: '#FFFFFF',
		},
		gray: {
			main: '#343a40', // Gray for the menu background
			light: '#495057',
			dark: '#212529',
			contrast: '#FFFFFF',
		},
	},
}

export const cssVariables = `
		--primary-main: ${theme.colors.primary.main};
		--primary-light: ${theme.colors.primary.light};
		--primary-dark: ${theme.colors.primary.dark};
		--primary-contrast: ${theme.colors.primary.contrast};
		--secondary-main: ${theme.colors.secondary.main};
		--secondary-light: ${theme.colors.secondary.light};
		--secondary-dark: ${theme.colors.secondary.dark};
		--secondary-contrast: ${theme.colors.secondary.contrast};
		--tertiary-main: ${theme.colors.tertiary.main};
		--tertiary-light: ${theme.colors.tertiary.light};
		--tertiary-dark: ${theme.colors.tertiary.dark};
		--tertiary-contrast: ${theme.colors.tertiary.contrast};
		--gray-main: ${theme.colors.gray.main};
		--gray-light: ${theme.colors.gray.light};
		--gray-dark: ${theme.colors.gray.dark};
		--gray-contrast: ${theme.colors.gray.contrast};
`

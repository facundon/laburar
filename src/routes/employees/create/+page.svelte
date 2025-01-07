<script lang="ts">
	import { invoke } from '$invoke'
	import { ROUTES } from '$routes'
	import { Employee } from '$models/employee'
	import Button from '$components/Button.svelte'
	import { Plus } from 'lucide-svelte'

	let firstName = ''
	let lastName = ''
	let address = ''
	let phone = ''
	let startDate: Date | undefined = undefined

	async function createEmployee() {
		try {
			const employee = new Employee({
				id: 0,
				firstName,
				lastName,
				address,
				phone,
				startDate,
				createdAt: new Date(),
			})
			await invoke('create_employee_command', employee.toCreateDTO())
			window.location.href = ROUTES.employee.list
		} catch (error) {
			console.error('Failed to create employee:', error)
		}
	}

	function handleDateChange(event: Event) {
		const startDateInput = event.target as HTMLInputElement
		startDate = startDateInput.value ? new Date(startDateInput.value) : undefined
	}
</script>

<main class="container">
	<h1>Agregar Personal</h1>
	<form onsubmit={createEmployee}>
		<div class="form-group">
			<label for="firstName">Nombre</label>
			<input id="firstName" bind:value={firstName} required />
		</div>
		<div class="form-group">
			<label for="lastName">Apellido</label>
			<input id="lastName" bind:value={lastName} required />
		</div>
		<div class="form-group">
			<label for="address">Dirección</label>
			<input id="address" bind:value={address} required />
		</div>
		<div class="form-group">
			<label for="phone">Teléfono</label>
			<input id="phone" bind:value={phone} />
		</div>
		<div class="form-group">
			<label for="startDate">Fecha de inicio</label>
			<input id="startDate" type="date" oninput={handleDateChange} />
		</div>
		<div class="actions">
			<Button outlined variant="secondary" href={ROUTES.employee.list}>Cancelar</Button>
			<Button type="submit" style="margin-left: auto;">
				<Plus style="margin-right: 5px;" />
				Crear
			</Button>
		</div>
	</form>
</main>

<style>
	.container {
		max-width: 600px;
		margin: 0 auto;
		padding: 2rem;
		background-color: #fff;
		border-radius: 8px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.actions {
		display: flex;
	}

	h1 {
		margin-bottom: 1rem;
	}

	.form-group {
		margin-bottom: 1rem;
	}

	label {
		display: block;
		margin-bottom: 0.5rem;
		font-weight: bold;
	}

	input {
		width: 100%;
		padding: 0.5rem;
		border: 1px solid #ccc;
		border-radius: 4px;
	}
</style>

<script lang="ts">
	import FormGroup from '$components/FormGroup.svelte'
	import Select from '$components/Select.svelte'
	import TextArea from '$components/TextArea.svelte'
	import { Task, TaskDifficulties } from '$models/task'

	interface Props {
		task: Task
		onsubmit: (event: Event) => void
		children: () => any
	}

	let { task = $bindable(), onsubmit = $bindable(), children }: Props = $props()
</script>

<form {onsubmit}>
	<div class="group">
		<FormGroup label="Nombre" id="name">
			<input id="name" bind:value={task.name} required />
		</FormGroup>
		<FormGroup label="Dificultad" id="dificulty">
			<Select id="dificulty" bind:value={task.difficulty} required>
				{#each TaskDifficulties as { label, value }}
					<option {value}>{label}</option>
				{/each}
			</Select>
		</FormGroup>
	</div>
	<div class="group">
		<FormGroup label="Area" id="area">
			<input id="area" bind:value={task.area} />
		</FormGroup>
		<FormGroup label="Frecuencia" id="frequency">
			<input id="frequency" bind:value={task.frequency} required />
		</FormGroup>
	</div>
	<div class="group">
		<FormGroup label="Descripción" id="description">
			<TextArea id="description" bind:value={task.description}></TextArea>
		</FormGroup>
	</div>
	{@render children()}
</form>

<style>
	.group {
		display: flex;
		gap: 3rem;
	}
</style>

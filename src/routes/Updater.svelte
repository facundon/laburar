<script lang="ts">
	import Modal from '$components/Modal.svelte'
	import { check, Update } from '@tauri-apps/plugin-updater'
	import { DownloadCloud } from 'lucide-svelte'
	import { onDestroy, onMount } from 'svelte'

	let status = $state<string | null>(null)
	let total = $state(0)
	let downloaded = $state(0)
	let percentage = $derived(((downloaded / total) * 100).toFixed(2))
	let updater = $state<Update | null>(null)

	function formatBytes(bytes: number) {
		const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
		if (bytes === 0) return '0 Byte'
		const i = Math.floor(Math.log(bytes) / Math.log(1024))
		return parseFloat((bytes / Math.pow(1024, i)).toFixed(2)) + ' ' + sizes[i]
	}

	async function checkUpdate() {
		const update = await check()
		if (!update) return
		status = 'Se encontró una actualización. Presiona el botón para descargarla.'
		updater = update
	}

	async function updateApp() {
		if (!updater) return
		await updater.downloadAndInstall(event => {
			switch (event.event) {
				case 'Started':
					total = event.data.contentLength || 0
					status = `Descargando ${updater?.version}...`
					break
				case 'Progress':
					downloaded += event.data.chunkLength
					break
				case 'Finished':
					status = 'Instalando...'
					break
			}
		})
	}

	let showModal = $state(false)

	const onConfirm = () => {
		updateApp()
		showModal = false
	}

	let interval = $state<number | null>(null)
	onMount(() => {
		checkUpdate()
		interval = setInterval(checkUpdate, 1000 * 60)
	})
	onDestroy(() => {
		if (interval) clearInterval(interval)
	})
</script>

{#if status}
	<div class="update-bar">
		<p>
			{status}{' '}
			{#if updater && total === 0}
				<DownloadCloud
					onclick={() => (showModal = true)}
					color="var(--success-light)"
					strokeWidth="1"
					style="margin-left: 0.5rem; vertical-align: middle; cursor: pointer;"
					size={'1.2rem'}
				/>
			{/if}
			{#if total > 0}
				<span>{percentage}% ({formatBytes(downloaded)} / {formatBytes(total)})</span>
			{/if}
		</p>
		<div class="progress" style="width: {percentage}%"></div>
	</div>
{/if}

<Modal show={showModal} onconfirm={onConfirm} onclose={() => (showModal = false)} title="Descargar e Instalar version {updater?.version}">
	<h3>Lista de cambios</h3>
	{#if updater?.body}
		<ul>
			{#each updater.body.split('\n') as line}
				<li>{line}</li>
			{/each}
		</ul>
	{:else}
		<p>No hay cambios disponibles.</p>
	{/if}
</Modal>

<style>
	h3 {
		margin: 0;
		font-weight: 600;
	}

	ul {
		padding-inline-start: 1rem;
	}

	li {
		margin-bottom: 0.5rem;
	}

	.update-bar {
		margin: auto;
		background-color: var(--gray-light);
		border-bottom-left-radius: 8px;
		border-bottom-right-radius: 8px;
		text-align: center;
		width: 90%;
		position: relative;
		z-index: 1;
	}

	.update-bar p {
		font-size: 0.8rem;
		gap: 2rem;
		color: white;
		padding-block: 0.25rem;
		margin: 0;
	}

	.update-bar p > span {
		font-weight: 600;
	}

	.progress {
		height: 100%;
		background: linear-gradient(270deg, rgb(18, 129, 79), #2e949b, #5e94ac, #aa44c4);
		background-size: 400% 400%;
		animation: gradientAnimation 5s ease infinite;
		border-bottom-left-radius: 8px;
		border-bottom-right-radius: 8px;
		position: absolute;
		top: 0;
		left: 0;
		z-index: -1;
		transition: width 0.5s ease;
	}

	@keyframes gradientAnimation {
		0% {
			background-position: 0% 50%;
		}
		50% {
			background-position: 100% 50%;
		}
		100% {
			background-position: 0% 50%;
		}
	}
</style>

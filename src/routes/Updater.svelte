<script lang="ts">
	import { check, Update } from '@tauri-apps/plugin-updater'
	import { DownloadCloud } from 'lucide-svelte'
	import { onDestroy, onMount } from 'svelte'

	let status = $state<string | null>(null)
	let total = $state(0)
	let downloaded = $state(0)
	let percentage = $derived(((downloaded / total) * 100).toFixed(2))
	let update = $state<Update | null>(null)

	function formatBytes(bytes: number) {
		const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
		if (bytes === 0) return '0 Byte'
		const i = Math.floor(Math.log(bytes) / Math.log(1024))
		return parseFloat((bytes / Math.pow(1024, i)).toFixed(2)) + ' ' + sizes[i]
	}

	async function checkUpdate() {
		console.log('checking for updates')
		const update = await check()
		console.log({ update })
		if (!update) return
		status = 'Se encontró una actualización. Presiona el botón para descargarla.'
	}

	async function updateApp() {
		if (!update) return
		await update.downloadAndInstall(event => {
			switch (event.event) {
				case 'Started':
					total = event.data.contentLength || 0
					status = `Descargando ${update.version}...`
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
		{#if status}
			<p>
				{status}{' '}
				{#if update && total === 0}
					<DownloadCloud
						onclick={updateApp}
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
		{/if}
		<div class="progress" style="width: {percentage}%"></div>
	</div>
{/if}

<style>
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

	p {
		font-size: 0.8rem;
		gap: 2rem;
		color: white;
		padding-block: 0.25rem;
		margin: 0;
	}

	p > span {
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
